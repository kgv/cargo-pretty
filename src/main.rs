//! Cargofmt

// TODO:
// - serde_diff:
//      https://github.com/amethyst/serde-diff/pull/17.
//      https://github.com/amethyst/serde-diff/issues/6
// - Inline Auto
//   если `key.len() + " = ".len() + inline_value.len()` <= max line len
// - package.description
// - подлежащее сказуемое (format, config show --path --toml)
// - Recursively searches

use self::options::{config::Config, Options, Output, SubCommand};
use anyhow::Result;
use cargofmt::{Format, Settings};
use log::{debug, info, trace};
use ron::ser::PrettyConfig;
use serde::Serialize;
use serde_diff::Diff;
use std::{
    borrow::Cow,
    fs::{read_to_string, rename, write, File},
    io::{stdout, Write},
    path::Path,
};
use toml_lalrpop::TomlParser;

// set RUST_LOG=info
//
// cargo run -- --backup --output=file Cargo.toml
//
// cargo run -- config active
//
// cargo run -- ./.rs.bk/sandbox/1/Cargo.toml
// cargo run -- ./assets/manifests/bitflags/Cargo.toml
// cargo run -- --backup .\\.rs.bk\\sandbox\\Cargo.toml
// cargo run -- --config-path=.\\ .\\.rs.bk\\sandbox\\Cargo.toml
// cargo run -- --backup --config-path=.\\ .\\.rs.bk\\sandbox\\Cargo.toml
fn main() -> Result<()> {
    env_logger::init();
    let options = Options::new();
    trace!("options: {:?}", options);
    let settings = settings(&options.config_path)?;
    trace!("settings: {:?}", settings);
    match options.subcommand {
        Some(SubCommand::Config(config)) => config_subcommand(&config, &settings)?,
        _ => command(&options, &settings)?,
    }
    Ok(())
}

fn config(path: &Path) -> Result<Option<Cow<Path>>> {
    const CONFIG_FILE_NAMES: [&str; 2] = ["cargofmt.toml", ".cargofmt.toml"];

    let mut dir = Cow::Borrowed(path);
    if dir.is_relative() {
        dir = Cow::Owned(dir.canonicalize()?);
    }
    while let Some(parent) = dir.parent() {
        for name in &CONFIG_FILE_NAMES {
            let file = dir.join(name);
            if file.exists() {
                return Ok(Some(Cow::Owned(file)));
            }
        }
        dir = Cow::Owned(parent.to_path_buf());
    }
    Ok(None)
}

fn settings(path: &Path) -> Result<Settings> {
    match config(path)? {
        Some(ref path) => {
            let content = read_to_string(path)?;
            info!("{} settings are used", path.display());
            Ok(toml::from_str(&content)?)
        }
        _ => {
            info!("default settings are used");
            Ok(Settings::default())
        }
    }
}

fn command(options: &Options, settings: &Settings) -> Result<()> {
    for manifest_file in &options.files {
        debug!("manifest_file: {}", manifest_file.display());
        let source = read_to_string(manifest_file)?;
        let mut manifest = TomlParser::new()
            .parse(&source)
            .map_err(|err| err.map_token(|token| token.to_string()))?;
        let target = manifest.format(&settings).to_string();
        let target = target.trim();
        match options.output {
            Output::Stdout => {
                println!("{}", target);
            }
            Output::File => {
                if source != target {
                    if options.backup {
                        let backup_file = manifest_file.with_extension("toml.bk");
                        rename(manifest_file, backup_file)?;
                    }
                    info!("manifest file was formated: {}", manifest_file.display());
                    write(manifest_file, target)?;
                }
            }
        }
    }
    Ok(())
}

fn config_subcommand(config: &Config, settings: &Settings) -> Result<()> {
    use self::options::config::{Format, Output, Type};
    use erased_serde::Serialize;

    let default = Settings::default();
    let mut writer: Box<dyn Write> = match &config.output {
        Output::Stdout => Box::new(stdout()),
        Output::File(path) => Box::new(File::create(path)?),
    };
    let serialize: Box<dyn Serialize> = match config.r#type {
        Type::Active => Box::new(settings),
        Type::Default => Box::new(default),
        Type::Diff => Box::new(Diff::serializable(&default, &settings)),
    };
    match config.format {
        Format::Json => json(&mut writer, serialize)?,
        Format::Ron => ron(&mut writer, serialize)?,
        Format::Toml => toml(&mut writer, serialize)?,
    }
    Ok(())
}

fn json<W: Write, T: Serialize>(writer: W, serialize: T) -> Result<()> {
    let mut serializer = serde_json::Serializer::pretty(writer);
    serialize.serialize(&mut serializer)?;
    Ok(())
}

fn ron<W: Write, T: Serialize>(writer: W, serialize: T) -> Result<()> {
    let mut serializer = ron::Serializer::new(writer, Some(PrettyConfig::new()), false)?;
    serialize.serialize(&mut serializer)?;
    Ok(())
}

fn toml<W: Write, T: Serialize>(mut writer: W, serialize: T) -> Result<()> {
    let mut buffer = String::with_capacity(8192);
    let mut serializer = toml::Serializer::pretty(&mut buffer);
    serializer.pretty_string(false);
    serialize.serialize(&mut serializer)?;
    write!(writer, "{}", buffer)?;
    Ok(())
}

mod options;

#[test]
fn test() {}
