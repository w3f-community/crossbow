use crate::error::*;
use std::path::{Path, PathBuf};
use std::process::Command;

// # Compile
/// AAPT2 supports compilation of all Android resource types, such as drawables and XML
/// files. When you invoke AAPT2 for compilation, you should pass a single resource file
/// as an input per invocation. AAPT2 then parses the file and generates an intermediate
/// binary file with a .flat extension.
///
/// Although you can pass resource directories containing more than one resource files to
/// AAPT2 using the --dir flag, you do not gain the benefits of incremental resource
/// compilation when doing so. That is, when passing whole directories, AAPT2 recompiles
/// all files in the directory even when only one resource has changed.
///
/// The output file types can differ based on the input you provide for compilation.
/// The files AAPT2 outputs are not executables and you must later include these binary
/// files as input in the link phase to generate an APK. However, the generated APK file
/// is not an executable that you can deploy on an Android device right away, as it does
/// not contain DEX files (compiled bytecode) and is not signed.
///
/// ## Compile syntax
/// The general syntax for using compile is as follows:
///
/// ```
/// aapt2 compile path-to-input-files [options] -o output-directory/
/// ```
/// ### Note
/// For resource files, the path to input files must match the following structure:
/// ```
/// path/resource-type[-config]/file
/// ```
///
/// In the following example, AAPT2 compiles resource files named values.xml and
/// myImage.png individually: ```
/// aapt2 compile project_root/module_root/src/main/res/values-en/strings.xml -o compiled/
/// aapt2 compile project_root/module_root/src/main/res/drawable/myImage.png -o compiled/
/// ```
///
/// As shown in the table above, the name of the output file depends on the input file
/// name and the name of its parent directory (the resource type and configuration).
/// For the example above with strings.xml as input, aapt2 automatically names the output
/// file as values-en_strings.arsc.flat. On the other hand, the file name for the compiled
/// drawable file stored in the drawable directory will be drawable_img.png.flat.
///
/// ## [Compile options](https://developer.android.com/studio/command-line/aapt2#compile_options)

#[derive(Clone)]
pub struct Aapt2Compile {
    inputs: Vec<PathBuf>,
    /// Specifies the output path for the compiled resource(s).
    ///
    /// This is a required flag because you must specify a path to a directory where AAPT2
    /// can output and store the compiled resources.
    o: PathBuf,
    /// Specifies the directory to scan for resources.
    ///
    /// Although you can use this flag to compile multiple resource files with one
    /// command, it disables the benefits of incremental compilation and thus, should not
    /// be used for large projects.
    dir: Option<PathBuf>,
    /// Zip file containing the res directory to scan for resources
    zip: Option<PathBuf>,
    /// Generates a text file containing the resource symbols in the specified file
    output_text_symbols: Option<String>,
    /// Generates pseudo-localized versions of default strings, such as en-XA and en-XB.
    pseudo_localize: bool,
    /// Disables PNG processing.
    ///
    /// Use this option if you have already processed the PNG files, or if you are
    /// creating debug builds that do not require file size reduction. Enabling this
    /// option results in a faster execution, but increases the output file size.
    no_crunch: bool,
    /// Treats errors that are permissible when using earlier versions of AAPT as
    /// warnings.
    ///
    /// This flag should be used for unexpected compile time errors.
    /// To resolve known behavior changes that you might get while using AAPT2, read
    /// [Behavior changes in AAPT2.](https://developer.android.com/studio/command-line/aapt2#aapt2_changes)
    legacy: bool,
    /// If specified, apply the same visibility rules for styleables as are used for all
    /// other resources. Otherwise, all stylesables will be made public.
    preserve_visibility_of_styleables: bool,
    /// Sets the visibility of the compiled resources to the specified level.
    /// Accepted levels: public, private, default.
    visibility: Option<Visibility>,
    /// Enable verbose logging.
    v: bool,
    /// Generate systrace json trace fragment to specified folder.
    trace_folder: Option<PathBuf>,
    /// Displays this help menu
    h: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Default,
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Public => write!(f, "public"),
            Self::Private => write!(f, "private"),
            Self::Default => write!(f, "default"),
        }
    }
}

impl Aapt2Compile {
    pub fn new(inputs: &[PathBuf], o: &Path) -> Self {
        Self {
            inputs: inputs.to_vec(),
            o: o.to_owned(),
            dir: None,
            zip: None,
            output_text_symbols: None,
            pseudo_localize: false,
            no_crunch: false,
            legacy: false,
            preserve_visibility_of_styleables: false,
            visibility: None,
            v: false,
            trace_folder: None,
            h: false,
        }
    }

    pub fn dir(&mut self, dir: &Path) -> &mut Self {
        self.dir = Some(dir.to_owned());
        self
    }

    pub fn zip(&mut self, zip: &Path) -> &mut Self {
        self.zip = Some(zip.to_owned());
        self
    }

    pub fn output_text_symbols(&mut self, output_text_symbols: String) -> &mut Self {
        self.output_text_symbols = Some(output_text_symbols.to_owned());
        self
    }

    pub fn pseudo_localize(&mut self) -> &mut Self {
        self.pseudo_localize = true;
        self
    }

    pub fn no_crunch(&mut self) -> &mut Self {
        self.no_crunch = true;
        self
    }

    pub fn legacy(&mut self) -> &mut Self {
        self.legacy = true;
        self
    }

    pub fn preserve_visibility_of_styleables(
        &mut self,
        preserve_visibility_of_styleables: bool,
    ) -> &mut Self {
        self.preserve_visibility_of_styleables = preserve_visibility_of_styleables;
        self
    }

    pub fn v(&mut self, v: bool) -> &mut Self {
        self.v = v;
        self
    }

    pub fn trace_folder(&mut self, trace_folder: &Path) -> &mut Self {
        self.trace_folder = Some(trace_folder.to_owned());
        self
    }

    pub fn h(&mut self, h: bool) -> &mut Self {
        self.h = h;
        self
    }

    pub fn visibility(&mut self, visibility: Visibility) -> &mut Self {
        self.visibility = Some(visibility);
        self
    }

    pub fn run(&self) -> Result<()> {
        let mut aapt2 = Command::new("aapt2");
        aapt2.arg("compile");
        self.inputs.iter().for_each(|input| {
            aapt2.arg(input);
        });
        aapt2.arg("-o");
        aapt2.arg(&self.o);
        if let Some(dir) = &self.dir {
            aapt2.arg("--dir").arg(dir);
        }
        if let Some(visibility) = &self.visibility {
            aapt2.arg("--visibility").arg(visibility.to_string());
        }
        if let Some(zip) = &self.zip {
            aapt2.arg("--zip").arg(zip);
        }
        if let Some(output_text_symbols) = &self.output_text_symbols {
            aapt2.arg("--output-text-symbols").arg(output_text_symbols);
        }
        if self.pseudo_localize {
            aapt2.arg("--pseudo-localize");
        }
        if self.no_crunch {
            aapt2.arg("--no-crunch");
        }
        if self.legacy {
            aapt2.arg("--legacy");
        }
        if self.preserve_visibility_of_styleables {
            aapt2.arg("--preserve-visibility-of-styleables");
        }
        if let Some(trace_folder) = &self.trace_folder {
            aapt2.arg("--trace-folder").arg(trace_folder);
        }
        if self.v {
            aapt2.arg("-v");
        }
        if self.h {
            aapt2.arg("-h");
        }
        aapt2.output_err(true)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_test_one() {
        let aapt2 = Aapt2Compile::new(
            &[Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\examples\\3d\\res\\android\\mipmap-hdpi\\ic_launcher.png").to_owned(),
            Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\examples\\3d\\res\\android\\mipmap-hdpi\\ic_launcher1.png").to_owned(),
            Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\examples\\3d\\res\\android\\mipmap-hdpi\\ic_launcher2.png").to_owned(),
            Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\examples\\3d\\res\\android\\mipmap-hdpi\\ic_launcher3.png").to_owned()],
            &Path::new("C:\\Users\\den99\\Desktop\\Work\\DodoRare\\creator\\examples\\3d\\res\\android\\mipmap-hdpi\\"),
        )
        .run();
    }
}
