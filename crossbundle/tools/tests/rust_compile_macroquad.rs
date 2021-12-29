use crossbundle_tools::{
    commands::{
        android::{self, Features},
        gen_minimal_project,
    },
    tools::{AndroidNdk, AndroidSdk},
    types::*,
};

#[test]
fn test_compile_android() {
    let tempdir = tempfile::tempdir().unwrap();
    let dir = tempdir.path();
    let _name = gen_minimal_project(dir).unwrap();

    let target_sdk_version = 30;
    let profile = Profile::Debug;
    let build_target = AndroidTarget::Aarch64LinuxAndroid;
    let sdk = AndroidSdk::from_env().unwrap();
    let ndk = AndroidNdk::from_env(Some(sdk.sdk_path())).unwrap();
    let lib_name = "test_lib";
    let features = Features::default();

    android::compile_rust_for_android_with_mq(
        &ndk,
        build_target,
        &dir,
        profile,
        features,
        target_sdk_version,
        lib_name,
    )
    .unwrap();
}
