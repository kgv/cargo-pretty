use crate::options::Pretty;
use anyhow::Result;
use cargo_pretty::{Format as _, Settings};
use log::{debug, info, trace};
use std::{
    borrow::Cow,
    fs::{read_to_string, rename, write},
    path::Path,
};
use toml_lalrpop::TomlParser;

pub(crate) fn pretty(pretty: &Pretty) -> Result<()> {
    use super::options::Output;

    let settings = settings(&pretty.config_path)?;
    trace!("settings: {:?}", settings);
    for manifest_file in &pretty.files {
        debug!("manifest_file: {}", manifest_file.display());
        let source = read_to_string(manifest_file)?;
        let mut manifest = TomlParser::new()
            .parse(&source)
            .map_err(|err| err.map_token(|token| token.to_string()))?;
        let target = manifest.format(&settings).to_string();
        let target = target.trim();
        trace!("output: {:?}", pretty.output);
        match pretty.output {
            Output::Stdout => {
                println!("{}", target);
            }
            Output::File => {
                if source != target {
                    trace!("backup: {:?}", pretty.backup);
                    if pretty.backup {
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

fn config(path: &Path) -> Result<Option<Cow<Path>>> {
    const CONFIG_FILE_NAMES: [&str; 2] = ["manifestfmt.toml", ".manifestfmt.toml"];

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

pub(crate) mod pretty {
    use super::settings;
    use crate::options::config::Config;
    use anyhow::Result;
    use cargo_pretty::Settings;
    use log::trace;
    use ron::ser::PrettyConfig;
    use serde::Serialize;
    use serde_diff::Diff;
    use std::{
        fs::File,
        io::{stdout, Write},
    };

    pub(crate) fn config(config: &Config) -> Result<()> {
        use crate::options::config::{Format, Output, Type};
        use erased_serde::Serialize;

        let default = Settings::default();
        let settings = settings(&config.config_path)?;
        trace!("settings: {:?}", settings);
        trace!("output: {:?}", config.output);
        let mut writer: Box<dyn Write> = match &config.output {
            Output::Stdout => Box::new(stdout()),
            Output::File(path) => Box::new(File::create(path)?),
        };
        trace!("type: {:?}", config.r#type);
        let serialize: Box<dyn Serialize> = match config.r#type {
            Type::Active => Box::new(settings),
            Type::Default => Box::new(default),
            Type::Diff => Box::new(Diff::serializable(&default, &settings)),
        };
        trace!("format: {:?}", config.format);
        match config.format {
            Format::Json => json(&mut writer, serialize)?,
            Format::Ron => ron(&mut writer, serialize)?,
            Format::Toml => toml(&mut writer, serialize)?,
        };
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
}
