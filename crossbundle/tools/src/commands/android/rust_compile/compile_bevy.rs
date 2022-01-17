use crate::{error::*, tools::*, types::*};
use cargo::{
    core::{
        self as cargo_core, compiler as cargo_compiler, manifest::TargetSourcePath, TargetKind,
        Workspace,
    },
    util::{CargoResult, Config as CargoConfig},
};
use cargo_util::*;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

/// Compiles rust code for android with bevy engine
pub fn compile_rust_for_android_with_bevy(
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
    let triple = build_target.rust_triple();

    // Takes clang and clang_pp paths
    let (clang, clang_pp) = ndk.clang(build_target, target_sdk_version)?;
    std::env::set_var(format!("CC_{}", triple), &clang);
    std::env::set_var(format!("CXX_{}", triple), &clang_pp);
    std::env::set_var(cargo_env_target_cfg("LINKER", triple), &clang);
    let ar = ndk.toolchain_bin("ar", build_target)?;
    std::env::set_var(format!("AR_{}", triple), &ar);

    // Specify path to workspace
    let cargo_config = CargoConfig::default()?;
    let workspace = Workspace::new(&project_path.join("Cargo.toml"), &cargo_config)?;

    // Define directory to build project
    let build_target_dir = workspace.root().join("target").join(triple).join(profile);
    std::fs::create_dir_all(&build_target_dir).unwrap();

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
    let lib_path = project_path.join("src").join("main.rs");
    let executor: Arc<dyn cargo_compiler::Executor> = Arc::new(DefaultExecutor {
        build_target_dir,
        lib_path,
    });

    // Compile all targets for the requested build target
    cargo::ops::compile_with_exec(&workspace, &opts, &executor)?;
    Ok(())
}

/// A `DefaultExecutor` calls rustc without doing anything else. It is Cargo's
/// default behaviour.
#[derive(Clone)]
pub struct DefaultExecutor {
    build_target_dir: PathBuf,
    lib_path: PathBuf,
}

impl cargo_compiler::Executor for DefaultExecutor {
    fn exec(
        &self,
        cmd: &ProcessBuilder,
        _id: cargo_core::PackageId,
        target: &cargo_core::Target,
        mode: cargo_compiler::CompileMode,
        on_stdout_line: &mut dyn FnMut(&str) -> CargoResult<()>,
        on_stderr_line: &mut dyn FnMut(&str) -> CargoResult<()>,
    ) -> CargoResult<()> {
        if mode == cargo_compiler::CompileMode::Build
            && (target.kind() == &TargetKind::Bin || target.kind() == &TargetKind::ExampleBin)
        {
            let mut cmd = cmd.clone();
            let ndk_glue_extra_code = super::consts::NDK_GLUE_EXTRA_CODE;
            let tmp_file =
                super::gen_tmp_lib_file::generate_lib_file(&self.lib_path, ndk_glue_extra_code)?;

            let mut new_args = cmd.get_args().to_owned();

            // Determine source path
            let path = if let TargetSourcePath::Path(path) = target.src_path() {
                path.to_owned()
            } else {
                // Ignore other values
                return Ok(());
            };
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
            std::fs::create_dir_all(&self.build_target_dir).unwrap();

            // Change crate-type from bin to cdylib
            // Replace output directory with the directory we created
            let mut iter = new_args.iter_mut().rev().peekable();
            while let Some(arg) = iter.next() {
                if let Some(prev_arg) = iter.peek() {
                    if *prev_arg == "--crate-type" && arg == "bin" {
                        *arg = "cdylib".into();
                    } else if *prev_arg == "--out-dir" {
                        *arg = self.build_target_dir.clone().into();
                    }
                }
            }
            cmd.args_replace(&new_args);
            cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
                .map(drop)
        } else {
            cmd.exec_with_streaming(on_stdout_line, on_stderr_line, false)
                .map(drop)
        }
    }
}

/// Helper function that allows to return environment argument with specified tool
fn cargo_env_target_cfg(tool: &str, target: &str) -> String {
    let utarget = target.replace('-', "_");
    let env = format!("CARGO_TARGET_{}_{}", &utarget, tool);
    env.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_android() {
        // Specify path to users directory
        let user_dirs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let project_path = user_dirs.parent().unwrap().parent().unwrap();

        // Specify path to bevy project example
        let project_path = project_path.join("examples").join("bevy-2d");

        // Assign needed configuration to compile rust for android with bevy
        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
        let build_target = AndroidTarget::Aarch64LinuxAndroid;
        let profile = Profile::Debug;
        let target_sdk_version = 30;
        let lib_name = "bevy_test_lib";

        // Compile rust code for android with bevy engine
        compile_rust_for_android_with_bevy(
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
