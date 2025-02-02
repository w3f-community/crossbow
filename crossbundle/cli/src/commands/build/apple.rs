use super::{BuildContext, SharedBuildCommand};
use crate::{error::*, types::CrossbowMetadata};
use apple_bundle::prelude::InfoPlist;
use clap::Parser;
use crossbundle_tools::{
    commands::{apple, combine_folders},
    types::*,
};
use std::path::{Path, PathBuf};

#[derive(Parser, Clone, Debug)]
pub struct IosBuildCommand {
    #[clap(flatten)]
    pub shared: SharedBuildCommand,
    /// Specify custom cargo binary.
    #[clap(long, conflicts_with = "example")]
    pub bin: Option<String>,
    /// Build for the given apple architecture.
    /// Supported targets are: `aarch64-apple-ios`, `aarch64-apple-ios-sim`,
    /// `armv7-apple-ios`, `armv7s-apple-ios`, `i386-apple-ios`, `x86_64-apple-ios`
    #[clap(long, short, multiple_values = true)]
    pub target: Vec<IosTarget>,
    /// Build strategy specifies what and how to build iOS application: with help of
    /// XCode, or with our native approach.
    #[clap(long, short, default_value = "native-ipa")]
    pub strategy: IosStrategy,
    /// Provisioning profile name to find in this directory:
    /// `~/Library/MobileDevice/Provisioning\ Profiles/`.
    #[clap(long, conflicts_with = "profile-path")]
    pub profile_name: Option<String>,
    /// Absolute path to provisioning profile.
    #[clap(long)]
    pub profile_path: Option<PathBuf>,
    /// The team identifier of your signing identity.
    #[clap(long)]
    pub team_identifier: Option<String>,
    /// The id of the identity used for signing. It won't start the signing process until
    /// you provide this flag.
    #[clap(long)]
    pub identity: Option<String>,
}

impl IosBuildCommand {
    pub fn run(&self, config: &Config) -> Result<()> {
        let context = BuildContext::new(config, self.shared.target_dir.clone())?;
        match &self.strategy {
            IosStrategy::NativeIpa => self.execute(config, &context)?,
        };
        Ok(())
    }

    pub fn execute(
        &self,
        config: &Config,
        context: &BuildContext,
    ) -> Result<(InfoPlist, Vec<PathBuf>)> {
        let project_path = &context.project_path;
        let profile = self.shared.profile();
        let (target, package_name) = if let Some(example) = &self.shared.example {
            (Target::Example(example.clone()), example.clone())
        } else if let Some(bin) = &self.bin {
            (Target::Bin(bin.clone()), bin.clone())
        } else {
            (Target::Bin(context.package_name()), context.package_name())
        };
        let properties = Self::gen_info_plist(context, &package_name)?;
        config.status_message("Starting build process", &package_name)?;
        config.status("Compiling app")?;
        let build_targets = Self::apple_build_targets(context, profile, &self.target);
        let mut app_paths = vec![];
        for build_target in build_targets {
            let app_path = self.build_app(
                config,
                context,
                target.clone(),
                project_path,
                build_target,
                &properties,
                profile,
                &package_name,
            )?;
            app_paths.push(app_path);
        }
        Ok((properties, app_paths))
    }

    fn build_app(
        &self,
        config: &Config,
        context: &BuildContext,
        target: Target,
        project_path: &Path,
        build_target: IosTarget,
        properties: &InfoPlist,
        profile: Profile,
        name: &str,
    ) -> Result<PathBuf> {
        let rust_triple = build_target.rust_triple();
        config.status_message("Compiling for architecture", rust_triple)?;
        apple::compile_rust_for_ios(
            target,
            build_target,
            project_path,
            profile,
            self.shared.features.clone(),
            self.shared.all_features,
            self.shared.no_default_features,
            &[],
        )?;
        let out_dir = context.target_dir.join(rust_triple).join(profile);
        let bin_path = out_dir.join(name);

        config.status("Generating app folder")?;
        let apple_target_dir = &context
            .target_dir
            .join("apple")
            .join(rust_triple)
            .join(profile);

        config.status("Preparing resources and assets")?;
        let (assets, resources) =
            Self::prepare_assets_and_resources(&context.config, apple_target_dir)?;

        let app_path = apple::gen_apple_app_folder(apple_target_dir, name, assets, resources)?;
        config.status("Copying binary to app folder")?;
        std::fs::copy(&bin_path, &app_path.join(name)).unwrap();
        config.status_message("Generating", "Info.plist")?;
        apple::save_info_plist(&app_path, properties, false).unwrap();

        if self.identity.is_some() {
            config.status("Starting code signing process")?;
            apple::copy_profile(
                &app_path,
                self.profile_name.clone(),
                self.profile_path.clone(),
            )?;
            config.status_message("Generating", "xcent file")?;
            let xcent_path = apple::gen_xcent(
                &app_path,
                name,
                self.team_identifier
                    .as_ref()
                    .ok_or(Error::TeamIdentifierNotProvided)?,
                &properties.identification.bundle_identifier,
                false,
            )?;
            config.status("Signing the binary")?;
            apple::codesign(&app_path.join(name), true, self.identity.clone(), None)?;
            config.status("Signing the bundle itself")?;
            apple::codesign(&app_path, true, self.identity.clone(), Some(xcent_path))?;
            config.status("Code signing process finished")?;
        }

        config.status("Generating ipa file")?;
        apple::gen_apple_ipa(apple_target_dir, &app_path, name)?;
        config.status("Build finished successfully")?;
        Ok(app_path)
    }

    /// Get apple build targets from cargo manifest
    pub fn apple_build_targets(
        context: &BuildContext,
        profile: Profile,
        build_targets: &Vec<IosTarget>,
    ) -> Vec<IosTarget> {
        if !build_targets.is_empty() {
            return build_targets.clone();
        }
        if profile == Profile::Debug && !context.config.apple.debug_build_targets.is_empty() {
            return context.config.apple.debug_build_targets.clone();
        }
        if profile == Profile::Release && !context.config.apple.release_build_targets.is_empty() {
            return context.config.apple.release_build_targets.clone();
        }
        vec![IosTarget::Aarch64Sim]
    }

    /// Get info plist from the path in cargo manifest or generate it with the given
    /// configuration
    pub fn gen_info_plist(context: &BuildContext, package_name: &str) -> Result<InfoPlist> {
        if let Some(info_plist_path) = &context.config.apple.info_plist_path {
            return Ok(apple::read_info_plist(info_plist_path)?);
        }
        let mut info_plist = if let Some(info_plist) = &context.config.apple.info_plist {
            info_plist.clone()
        } else {
            InfoPlist::default()
        };
        update_info_plist_with_default(
            &mut info_plist,
            package_name,
            context.config.app_name.clone(),
        );
        context.config.permissions.iter().for_each(|permission| {
            permission.update_info_plist(&mut info_plist);
        });
        Ok(info_plist)
    }

    /// Prepare assets and resources for the application.
    pub fn prepare_assets_and_resources(
        config: &CrossbowMetadata,
        out_dir: &Path,
    ) -> Result<(Option<PathBuf>, Option<PathBuf>)> {
        let res = config.get_apple_resources();
        let gen_resources = if res.is_empty() && config.icon.is_none() {
            None
        } else {
            let path = out_dir.join("gen_resources");
            std::fs::remove_dir_all(&path).ok();
            combine_folders(res, &path)?;

            // TODO: Generate icons
            Some(path)
        };

        let assets = config.get_apple_assets();
        let gen_assets = if !res.is_empty() {
            let path = out_dir.join("gen_assets");
            std::fs::remove_dir_all(&path).ok();
            combine_folders(assets, &path)?;
            Some(path)
        } else {
            None
        };
        Ok((gen_assets, gen_resources))
    }
}
