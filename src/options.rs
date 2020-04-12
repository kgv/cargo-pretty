use self::config::Config;
use anyhow::{bail, Error, Result};
use clap::{AppSettings, Clap};
use std::{convert::TryFrom, ffi::OsStr, path::PathBuf, str::FromStr};

#[derive(Clap, Debug)]
#[clap(version, author, about)]
#[clap(setting = AppSettings::ArgsNegateSubcommands)]
#[clap(setting = AppSettings::SubcommandsNegateReqs)]
#[clap(setting = AppSettings::VersionlessSubcommands)]
pub struct Options {
    /// Recursively searches the path for the cargofmt.toml config file
    #[clap(name = "CONFIG_PATH", long = "config-path", default_value = "./")]
    pub config_path: PathBuf,
    /// Output type
    #[clap(name = "OUTPUT", long = "output", default_value = "stdout", possible_values = &["file", "stdout"])]
    pub output: Output,

    /// Backup any modified files
    #[clap(short, long = "backup")]
    pub backup: bool,

    /// Sets the manifest files to format
    #[clap(name = "MANIFEST_FILES", default_value = "Cargo.toml", parse(from_os_str))]
    pub files: Vec<PathBuf>,

    #[clap(subcommand)]
    pub subcommand: Option<SubCommand>,
}

impl Options {
    pub fn new() -> Options {
        let options = Options::parse();
        for file in &options.files {
            assert!(file.is_file(), "isn't file");
            assert!(
                Some(OsStr::new("Cargo.toml")) == file.file_name(),
                "file isn't Cargo.toml"
            );
        }
        options
    }
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    #[clap(version, author)]
    Config(Config),
}

/// Output type
#[derive(Debug)]
pub enum Output {
    File,
    Stdout,
}

impl TryFrom<&str> for Output {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        match from {
            "file" => Ok(Self::File),
            "stdout" => Ok(Self::Stdout),
            _ => bail!("can't parse output from the string: {}", from),
        }
    }
}

impl FromStr for Output {
    type Err = Error;

    fn from_str(from: &str) -> Result<Self, Self::Err> {
        Self::try_from(from)
    }
}

pub mod config {
    use anyhow::{bail, Error, Result};
    use clap::Clap;
    use std::{convert::TryFrom, ffi::OsStr, path::PathBuf, str::FromStr};

    /// Manipulate config
    #[derive(Clap, Debug)]
    pub struct Config {
        /// Config type
        #[clap(name = "TYPE", default_value = "active", possible_values = &["active", "default", "diff"])]
        pub r#type: Type,
        /// Format type
        #[clap(name = "FORMAT", long = "format", default_value = "toml", possible_values = &["json", "ron", "toml"])]
        pub format: Format,
        /// Output type
        #[clap(
            name = "OUTPUT",
            long = "output",
            default_value = "stdout",
            parse(from_os_str)
        )]
        pub output: Output,
    }

    /// Format
    #[derive(Debug)]
    pub enum Format {
        Json,
        Ron,
        Toml,
    }

    impl TryFrom<&str> for Format {
        type Error = Error;

        fn try_from(from: &str) -> Result<Self, Self::Error> {
            match from {
                "json" => Ok(Self::Json),
                "ron" => Ok(Self::Ron),
                "toml" => Ok(Self::Toml),
                _ => bail!("can't parse format from the string: {}", from),
            }
        }
    }

    impl FromStr for Format {
        type Err = Error;

        fn from_str(from: &str) -> Result<Self, Self::Err> {
            Self::try_from(from)
        }
    }

    /// Output
    #[derive(Debug)]
    pub enum Output {
        File(PathBuf),
        Stdout,
    }

    impl Default for Output {
        fn default() -> Self {
            Self::Stdout
        }
    }

    impl From<&OsStr> for Output {
        fn from(from: &OsStr) -> Self {
            match from.to_str() {
                Some("stdout") => Self::Stdout,
                _ => Self::File(PathBuf::from(from)),
            }
        }
    }

    /// Config type
    #[derive(Clap, Debug)]
    pub enum Type {
        /// Active config
        Active,
        /// Default config
        Default,
        /// Difference between default and active configs
        Diff,
    }

    impl Default for Type {
        fn default() -> Self {
            Self::Active
        }
    }

    impl TryFrom<&str> for Type {
        type Error = Error;

        fn try_from(from: &str) -> Result<Self, Self::Error> {
            match from {
                "active" => Ok(Self::Active),
                "default" => Ok(Self::Default),
                "diff" => Ok(Self::Diff),
                _ => bail!("can't parse config type from the string: {}", from),
            }
        }
    }

    impl FromStr for Type {
        type Err = Error;

        fn from_str(from: &str) -> Result<Self, Self::Err> {
            Self::try_from(from)
        }
    }
}
