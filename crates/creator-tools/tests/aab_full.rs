use creator_tools::{
    commands::{android, gen_minimal_project},
    tools::*,
    types::*,
};

#[cfg(test)]
mod tests {

    use creator_tools::commands::android::gen_debug_key;

    use super::*;

    #[test]
    fn test_aab() {
        // Create temporary directory
        let tempdir = tempfile::tempdir().unwrap();
        let project_path = tempdir.path();
        let package_name = gen_minimal_project(&project_path).unwrap();

        let sdk = AndroidSdk::from_env().unwrap();
        let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();

        let target_sdk_version = 30;
        let profile = Profile::Debug;
        let build_target = AndroidTarget::Aarch64LinuxAndroid;
        android::compile_rust_for_android(
            &ndk,
            Target::Lib,
            build_target,
            &project_path,
            profile,
            vec![],
            false,
            false,
            target_sdk_version,
        )
        .unwrap();
        let target_dir = project_path.join("target");
        let out_dir = target_dir
            .join(build_target.rust_triple())
            .join(profile.as_ref());
        let compiled_lib = out_dir.join(format!("lib{}.so", package_name));
        assert!(compiled_lib.exists());

        let android_build_dir = target_dir.join("android").join(profile.to_string());

        let android_abi = build_target.android_abi();
        let android_compiled_lib = android_build_dir
            .join("lib")
            .join(android_abi)
            .join(format!("lib{}.so", package_name));
        if !android_compiled_lib.exists() {
            std::fs::create_dir_all(&android_compiled_lib.parent().unwrap()).unwrap();
            fs_extra::file::copy(
                &compiled_lib,
                &android_compiled_lib,
                &fs_extra::file::CopyOptions::new(),
            )
            .unwrap();
        }

        // Generate manifest
        let manifest = android::gen_minimal_android_manifest(
            &package_name,
            None,
            "0.0.1".to_string(),
            None,
            None,
            target_sdk_version,
            None,
            None,
        );
        let manifest_path = android::save_android_manifest(&android_build_dir, &manifest).unwrap();
        assert!(manifest_path.exists());

        // Gen apks and prepare modules (zip, zip, zip)
        let base_apk_path = android::gen_base_aab_module(
            None,
            None,
            &android_build_dir,
            &sdk,
            None,
            None,
            false,
            &package_name,
            &manifest_path,
            Some(28),
            target_sdk_version,
            None,
            None,
            false,
            None,
            false,
            false,
            false,
            false,
        )
        .unwrap();
        assert!(base_apk_path.exists());
        if manifest_path.exists() {
            std::fs::remove_file(manifest_path).unwrap();
        }

        // Assign path to lib
        let add_lib = android::add_libs_into_aapt2(
            &ndk,
            &android_compiled_lib,
            build_target,
            profile,
            30,
            &base_apk_path,
            &target_dir,
        )
        .unwrap();
        assert!(add_lib.exists());

        let gen_zip_modules =
            android::gen_zip_modules(&android_build_dir, &package_name, &base_apk_path).unwrap();

        // Gen aab from given list of modules (zip, zip, zip)
        let aab_path =
            android::gen_aab_from_modules(&package_name, &[gen_zip_modules], &android_build_dir)
                .unwrap();
        assert!(aab_path.exists());
        for entry in std::fs::read_dir(&android_build_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.ends_with("compiled_res") {
                std::fs::remove_dir_all(path.clone()).unwrap();
            }
            if path.ends_with("extracted_apk_files") {
                std::fs::remove_dir_all(path.clone()).unwrap();
            }
            if path.ends_with("example_module.zip") {
                std::fs::remove_file(path).unwrap();
            }
        }

        // Create keystore with keytool command
        let key = gen_debug_key().unwrap();
        println!("{:?}", key);

        // Create keystore with keytool command
        let apks = android_build_dir.join(format!("{}.apks", package_name));
        let build_apks = android::build_apks(&aab_path, &apks, &package_name, key).unwrap();
    }
}
