use anyhow::Result;
use cargofmt::{Format, Settings};
use toml_lalrpop::TomlParser;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn badges() -> Result<()> {
        let source = include_str!(".././assets/bages/source/Cargo.toml");
        let target = include_str!(".././assets/bages/target/Cargo.toml");
        let mut manifest = TomlParser::new().parse(&source).unwrap();
        let settings = Settings::default();
        let formated = format!("{}", manifest.format(&settings));
        assert_eq!(formated, target);
        Ok(())
    }
}
