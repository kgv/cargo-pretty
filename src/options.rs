use anyhow::{bail, Error, Result};
use clap::Clap;
use std::{convert::TryFrom, ffi::OsStr, path::PathBuf, str::FromStr};

pub fn parse() -> Options {
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

#[derive(Clap, Debug)]
#[clap(version, author, about)]
pub struct Options {
    /// Run in 'check' mode.
    #[clap(short = "c", long = "check")]
    pub check: bool,
    /// What data to emit and how.
    #[clap(name = "EMIT", long = "emit", possible_values = &["files", "stdout", "coverage", "checkstyle", "json"])]
    pub emit: Option<Emit>,
    /// Backup any modified files.
    #[clap(short = "b", long = "backup")]
    pub backup: bool,
    /// Recursively searches the given path for the cargofmt.toml config file.
    #[clap(name = "CONFIG_PATH", long = "config-path", default_value = "./")]
    pub config_path: PathBuf,
    /// Sets the manifest files to format.
    #[clap(name = "MANIFEST_FILES", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}

#[derive(Debug)]
pub enum Emit {
    Files,
    Stdout,
    Coverage,
    CheckStyle,
    Json,
}

impl<'a> TryFrom<&'a str> for Emit {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        match from {
            "files" => Ok(Self::Files),
            "stdout" => Ok(Self::Stdout),
            "coverage" => Ok(Self::Coverage),
            "checkstyle" => Ok(Self::CheckStyle),
            "json" => Ok(Self::Json),
            _ => bail!("can't parse emit from the string: {}", from),
        }
    }
}

impl FromStr for Emit {
    type Err = Error;

    fn from_str(from: &str) -> Result<Self, Self::Err> {
        Self::try_from(from)
    }
}

#[derive(Debug)]
pub enum PrintConfig {
    Default,
    Current,
}

impl<'a> TryFrom<&'a str> for PrintConfig {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        match from {
            "default" => Ok(Self::Default),
            "current" => Ok(Self::Current),
            _ => bail!("can't parse print-config from the string: {}", from),
        }
    }
}

impl FromStr for PrintConfig {
    type Err = Error;

    fn from_str(from: &str) -> Result<Self, Self::Err> {
        Self::try_from(from)
    }
}
