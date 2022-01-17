use crate::error::*;
use crate::tools::*;
use crate::types::*;
use cargo::{
    core::{
        compiler::{CompileMode, Executor},
        manifest::TargetSourcePath,
        {PackageId, TargetKind, Workspace},
    },
    util::{CargoResult, Config as CargoConfig},
};
use cargo_util::ProcessBuilder;
use std::{
    ffi::{OsStr, OsString},
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};
use tempfile::NamedTempFile;

/// Compiles rust code for android with macroquad engine
pub fn compile_rust_for_android_with_mq(
    ndk: &AndroidNdk,
    build_target: AndroidTarget,
    project_path: &Path,
    profile: Profile,
    features: Vec<String>,
    all_features: bool,
    no_default_features: bool,
    target_sdk_version: u32,
    lib_name: &str,
) -> Result<()> {
    // Specify path to workspace
    let cargo_config = CargoConfig::default()?;
    let workspace = Workspace::new(&project_path.join("Cargo.toml"), &cargo_config)?;
    let rust_triple = build_target.rust_triple();

    // Define directory to build project
    let build_target_dir = workspace
        .root()
        .join("target")
        .join(rust_triple)
        .join(profile);
    fs::create_dir_all(&build_target_dir).unwrap();

    // Set environment variables needed for use with the cc crate
    let (clang, clang_pp) = ndk.clang(build_target, target_sdk_version)?;
    let ar = ndk.toolchain_bin("ar", build_target)?;

    std::env::set_var(format!("CC_{}", rust_triple), &clang);
    std::env::set_var(format!("CXX_{}", rust_triple), &clang_pp);
    std::env::set_var(format!("AR_{}", rust_triple), &ar);

    set_cmake_vars(build_target, ndk, target_sdk_version, &build_target_dir)?;

    // Use libc++. It is current default C++ runtime
    std::env::set_var("CXXSTDLIB", "c++");

    // Configure compilation options so that we will build the desired build_target
    let opts = super::compile_options::compile_options(
        &workspace,
        build_target,
        features,
        all_features,
        no_default_features,
        &build_target_dir,
        lib_name,
        profile,
    )?;

    // Create the executor
    let executor: Arc<dyn Executor> = Arc::new(SharedLibraryExecutor {
        ndk: ndk.clone(),
        target_sdk_version,
        profile,
        build_target_dir,
        build_target,
        nostrip: false,
    });

    // Compile all targets for the requested build target
    cargo::ops::compile_with_exec(&workspace, &opts, &executor)?;
    Ok(())
}

/// Helper function for Executor trait. Match compile mode and return cmd arguments
fn exec_compilation(
    cmd: &ProcessBuilder,
    build_target_dir: &Path,
    build_target: AndroidTarget,
    ndk: &AndroidNdk,
    target_sdk_version: u32,
    nostrip: bool,
    profile: Profile,
    sokol_extra_code: &'static str,
    target: &cargo::core::Target,
    mode: CompileMode,
    on_stdout_line: &mut dyn FnMut(&str) -> CargoResult<()>,
    on_stderr_line: &mut dyn FnMut(&str) -> CargoResult<()>,
) -> CargoResult<()> {
    if mode == CompileMode::Build
        && (target.kind() == &TargetKind::Bin || target.kind() == &TargetKind::ExampleBin)
    {
        // Determine source path
        let path = if let TargetSourcePath::Path(path) = target.src_path() {
            path.to_owned()
        } else {
            // Ignore other values
            return Ok(());
        };

        // Generate source file that will be built
        let tmp_file = super::gen_tmp_lib_file::generate_lib_file(&path, sokol_extra_code)?;

        // Replaces source argument and returns collection of arguments
        get_cmd_args(
            &path,
            tmp_file,
            build_target_dir,
            target,
            cmd,
            ndk,
            &build_target,
            target_sdk_version,
            nostrip,
            profile,
            on_stdout_line,
            on_stderr_line,
        )?;
    } else if mode == CompileMode::Test {
        // This occurs when --all-targets is specified
        return Err(anyhow::Error::msg(format!(
            "Ignoring CompileMode::Test for target: {}",
            target.name()
        )));
    } else if mode == CompileMode::Build {
        let mut new_args = cmd.get_args().to_owned();

        // Change crate-type from cdylib to rlib
        let mut iter = new_args.iter_mut().rev().peekable();
        while let Some(arg) = iter.next() {
            if let Some(prev_arg) = iter.peek() {
                if *prev_arg == "--crate-type" && arg == "cdylib" {
                    *arg = "rlib".into();
                }
            }
        }

        let mut cmd = cmd.clone();
        cmd.args_replace(&new_args);
        cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
            .map(drop)?
    } else {
        cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
            .map(drop)?
    }

    Ok(())
}

/// Executor which builds binary and example targets as static libraries
struct SharedLibraryExecutor {
    ndk: AndroidNdk,
    target_sdk_version: u32,
    build_target_dir: PathBuf,
    build_target: AndroidTarget,
    profile: Profile,
    nostrip: bool,
}

impl Executor for SharedLibraryExecutor {
    fn exec(
        &self,
        cmd: &ProcessBuilder,
        _id: PackageId,
        target: &cargo::core::Target,
        mode: CompileMode,
        on_stdout_line: &mut dyn FnMut(&str) -> CargoResult<()>,
        on_stderr_line: &mut dyn FnMut(&str) -> CargoResult<()>,
    ) -> CargoResult<()> {
        let sokol_extra_code = super::consts::SOKOL_EXTRA_CODE;
        exec_compilation(
            cmd,
            &self.build_target_dir,
            self.build_target,
            &self.ndk,
            self.target_sdk_version,
            self.nostrip,
            self.profile,
            sokol_extra_code,
            target,
            mode,
            on_stdout_line,
            on_stderr_line,
        )
    }
}

/// Write a CMake toolchain which will remove references to the rustc build target before
/// including the NDK provided toolchain. The NDK provided android toolchain will set the
/// target appropriately Returns the path to the generated toolchain file
fn write_cmake_toolchain(
    min_sdk_version: u32,
    // ndk_path: &AndroidNdk,
    // make sure ndk_path is valid
    ndk_path: &Path,
    build_target_dir: &Path,
    build_target: AndroidTarget,
) -> CargoResult<PathBuf> {
    let toolchain_path = build_target_dir.join("cargo-apk.toolchain.cmake");
    let mut toolchain_file = fs::File::create(&toolchain_path).unwrap();
    writeln!(
        toolchain_file,
        r#"set(ANDROID_PLATFORM android-{min_sdk_version})
        set(ANDROID_ABI {abi})
        string(REPLACE "--target={build_target}" "" CMAKE_C_FLAGS "${{CMAKE_C_FLAGS}}")
        string(REPLACE "--target={build_target}" "" CMAKE_CXX_FLAGS "${{CMAKE_CXX_FLAGS}}")
        unset(CMAKE_C_COMPILER CACHE)
        unset(CMAKE_CXX_COMPILER CACHE)
        include("{ndk_path}/build/cmake/android.toolchain.cmake")"#,
        min_sdk_version = min_sdk_version,
        ndk_path = ndk_path.to_string_lossy().replace("\\", "/"), /* Use forward slashes even on
                                                                   * windows to avoid path
                                                                   * escaping issues. */
        build_target = build_target.rust_triple(),
        abi = build_target.android_abi(),
    )?;
    Ok(toolchain_path)
}

/// Returns path to NDK provided make
fn make_path(ndk_path: &Path) -> PathBuf {
    ndk_path
        .join("prebuild")
        .join(super::consts::HOST_TAG)
        .join("make")
}

/// Get the program arguments and execute program with it
fn get_cmd_args(
    path: &Path,
    tmp_file: NamedTempFile,
    build_target_dir: &Path,
    target: &cargo::core::Target,
    cmd: &ProcessBuilder,
    ndk: &AndroidNdk,
    build_target: &AndroidTarget,
    target_sdk_version: u32,
    nostrip: bool,
    profile: Profile,
    on_stdout_line: &mut dyn FnMut(&str) -> CargoResult<()>,
    on_stderr_line: &mut dyn FnMut(&str) -> CargoResult<()>,
) -> CargoResult<()> {
    // Get the program arguments
    let mut new_args = cmd.get_args().to_owned();

    // Replace source argument
    let filename = path.file_name().unwrap().to_owned();
    let source_arg = new_args.iter_mut().find_map(|arg| {
        let path_arg = Path::new(&arg);
        let tmp = path_arg.file_name().unwrap();

        if filename == tmp {
            Some(arg)
        } else {
            None
        }
    });

    if let Some(source_arg) = source_arg {
        // Build a new relative path to the temporary source file and use it as the source
        // argument Using an absolute path causes compatibility issues in
        // some cases under windows If a UNC path is used then relative
        // paths used in "include* macros" may not work if the relative path
        // includes "/" instead of "\"
        let path_arg = Path::new(&source_arg);
        let mut path_arg = path_arg.to_path_buf();
        path_arg.set_file_name(tmp_file.path().file_name().unwrap());
        *source_arg = path_arg.into_os_string();
    } else {
        return Err(anyhow::Error::msg(format!(
            "Unable to replace source argument when building target: {}",
            target.name()
        )));
    }

    // Create output directory inside the build target directory
    let build_path = build_target_dir.to_path_buf();
    fs::create_dir_all(&build_path).unwrap();

    // Change crate-type from bin to cdylib
    // Replace output directory with the directory we created
    let mut iter = new_args.iter_mut().rev().peekable();
    while let Some(arg) = iter.next() {
        if let Some(prev_arg) = iter.peek() {
            if *prev_arg == "--crate-type" && arg == "bin" {
                *arg = "cdylib".into();
            } else if *prev_arg == "--out-dir" {
                *arg = build_path.clone().into();
            }
        }
    }

    // Determine paths
    let tool_root = ndk.toolchain_dir().unwrap();
    let linker_path = tool_root
        .join("bin")
        .join(format!("{}-ld.gold", build_target.ndk_triple()));
    let sysroot = tool_root.join("sysroot");
    let version_independent_libraries_path = sysroot
        .join("usr")
        .join("lib")
        .join(build_target.ndk_triple());
    let version_specific_libraries_path =
        AndroidNdk::find_ndk_path(target_sdk_version, |platform| {
            version_independent_libraries_path.join(platform.to_string())
        })
        .map_err(|_| anyhow::Error::msg("Android SDK not found"))?;
    let gcc_lib_path = tool_root
        .join("lib/gcc")
        .join(build_target.ndk_triple())
        .join("4.9.x");

    // Add linker arguments
    // Specify linker
    new_args.push(build_arg("-Clinker=", linker_path));

    // Set linker flavor
    new_args.push("-Clinker-flavor=ld".into());

    // Set system root
    new_args.push(build_arg("-Clink-arg=--sysroot=", sysroot));

    // Add version specific libraries directory to search path
    new_args.push(build_arg("-Clink-arg=-L", &version_specific_libraries_path));

    // Add version independent libraries directory to search path
    new_args.push(build_arg(
        "-Clink-arg=-L",
        &version_independent_libraries_path,
    ));

    // Add path to folder containing libgcc.a to search path
    new_args.push(build_arg("-Clink-arg=-L", gcc_lib_path));

    // Strip symbols for release builds
    if !nostrip && profile == Profile::Release {
        new_args.push("-Clink-arg=-strip-all".into());
    }

    // Require position independent code
    new_args.push("-Crelocation-model=pic".into());

    // Create new command
    let mut cmd = cmd.clone();
    cmd.args_replace(&new_args);

    // Execute the command
    cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
        .map(drop)?;

    Ok(())
}

/// Helper function to build arguments composed of concatenating two strings
fn build_arg(start: &str, end: impl AsRef<OsStr>) -> OsString {
    let mut new_arg = OsString::new();
    new_arg.push(start);
    new_arg.push(end.as_ref());
    new_arg
}

/// Sets needed environment variables
fn set_cmake_vars(
    build_target: AndroidTarget,
    ndk: &AndroidNdk,
    target_sdk_version: u32,
    build_target_dir: &Path,
) -> CargoResult<()> {
    // Return path to toolchain cmake file
    let cmake_toolchain_path = write_cmake_toolchain(
        target_sdk_version,
        ndk.ndk_path(),
        build_target_dir,
        build_target,
    )?;

    // Set cmake environment variables
    std::env::set_var("CMAKE_TOOLCHAIN_FILE", cmake_toolchain_path);
    std::env::set_var("CMAKE_GENERATOR", r#"Unix Makefiles"#);
    std::env::set_var("CMAKE_MAKE_PROGRAM", make_path(ndk.ndk_path()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_android() {
        // Specify path to users directory
        let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let project_path = user_dirs.parent().unwrap().parent().unwrap();

        // Specify path to macroquad project example
        let project_path = project_path.join("examples").join("macroquad-3d");

        // Assign needed configuration to compile rust for android with macroquad
        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
        let build_target = AndroidTarget::Aarch64LinuxAndroid;
        let profile = Profile::Debug;
        let target_sdk_version = 30;
        let lib_name = "mq_test_lib";

        // Compile rust code for android with macroquad engine
        compile_rust_for_android_with_mq(
            &ndk,
            build_target,
            &project_path,
            profile,
            vec![],
            false,
            false,
            target_sdk_version,
            lib_name,
        )
        .unwrap();
    }
}