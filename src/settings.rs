use crate::{inline::Kind as InlineKind, order::Kind as OrderKind};
use derivative::Derivative;
use indexmap::indexmap;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

const CONFIG_FILE_NAMES: [&str; 2] = [".cargofmt.toml", "cargofmt.toml"];

lazy_static! {
    pub static ref DEFAULT: Settings = Settings::default();
    pub static ref ROOT: OrderKind = {
        OrderKind::Enumeration(indexmap! {
            Cow::Borrowed("package") => 0,
            Cow::Borrowed("lib") => 1,
            Cow::Borrowed("bin") => 2,
            Cow::Borrowed("example") => 3,
            Cow::Borrowed("test") => 4,
            Cow::Borrowed("bench") => 5,
            Cow::Borrowed("dependencies") => 6,
            Cow::Borrowed("dev-dependencies") => 7,
            Cow::Borrowed("build-dependencies") => 8,
            Cow::Borrowed("target") => 9,
            Cow::Borrowed("badges") => 10,
            Cow::Borrowed("features") => 11,
            Cow::Borrowed("replace") => 12,
            Cow::Borrowed("patch") => 13,
            Cow::Borrowed("replace") => 14,
            Cow::Borrowed("profile") => 15,
            Cow::Borrowed("workspace") => 16,
        })
    };
    pub static ref PACKAGE: OrderKind = {
        OrderKind::Enumeration(indexmap! {
            Cow::Borrowed("name") => 0,
            Cow::Borrowed("version") => 1,
            Cow::Borrowed("authors") => 2,
            Cow::Borrowed("edition") => 3,
            Cow::Borrowed("description") => 4,
            Cow::Borrowed("documentation") => 5,
            Cow::Borrowed("readme") => 6,
            Cow::Borrowed("homepage") => 7,
            Cow::Borrowed("repository") => 8,
            Cow::Borrowed("license") => 9,
            Cow::Borrowed("license-file") => 10,
            Cow::Borrowed("keywords") => 11,
            Cow::Borrowed("categories") => 12,
            Cow::Borrowed("workspace") => 13,
            Cow::Borrowed("build") => 14,
            Cow::Borrowed("links") => 15,
            Cow::Borrowed("exclude") => 16,
            Cow::Borrowed("include") => 17,
            Cow::Borrowed("publish") => 18,
            Cow::Borrowed("metadata") => 19,
            Cow::Borrowed("default-run") => 20,
            Cow::Borrowed("autobins") => 21,
            Cow::Borrowed("autoexamples") => 22,
            Cow::Borrowed("autotests") => 23,
            Cow::Borrowed("autobenches") => 24,
        })
    };
    pub static ref TARGET: OrderKind = {
        OrderKind::Enumeration(indexmap! {
            Cow::Borrowed("name") => 0,
            Cow::Borrowed("path") => 1,
            Cow::Borrowed("test") => 2,
            Cow::Borrowed("doctest") => 3,
            Cow::Borrowed("bench") => 4,
            Cow::Borrowed("doc") => 5,
            Cow::Borrowed("plugin") => 6,
            Cow::Borrowed("proc-macro") => 7,
            Cow::Borrowed("harness") => 8,
            Cow::Borrowed("edition") => 9,
            Cow::Borrowed("crate-type") => 10,
            Cow::Borrowed("required-features") => 11,
        })
    };
}

/// Settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct Settings {
    #[derivative(Default(value = "ROOT.clone()"))]
    pub order: OrderKind,
    pub package: Package,
    pub lib: Lib,
    pub bin: Bin,
    pub example: Example,
    pub test: Test,
    pub bench: Bench,
    pub dependencies: Dependencies,
    pub build_dependencies: BuildDependencies,
    pub dev_dependencies: DevDependencies,
    pub target: Target,
    pub badges: Badges,
    pub features: Features,
    pub patch: Patch,
    pub replace: Replace,
    pub profile: Profile,
    pub workspace: Workspace,
}

/// The package settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields)]
pub struct Package {
    #[derivative(Default(value = "PACKAGE.clone()"))]
    pub order: OrderKind,
    #[derivative(Default(value = "InlineKind::Manual(Some(1))"))]
    pub inline: InlineKind,
    pub authors: Order,
    pub keywords: Order,
    pub categories: Order,
    pub exclude: Order,
    pub include: Order,
}

// Target tables:

/// The lib settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct Lib {
    #[derivative(Default(value = "TARGET.clone()"))]
    pub order: OrderKind,
    pub crate_type: Order,
}

/// The bin settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct Bin {
    #[derivative(Default(value = "TARGET.clone()"))]
    pub order: OrderKind,
    pub required_features: Order,
}

/// The example settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct Example {
    #[derivative(Default(value = "TARGET.clone()"))]
    pub order: OrderKind,
    pub required_features: Order,
}

/// The test settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct Test {
    #[derivative(Default(value = "TARGET.clone()"))]
    pub order: OrderKind,
    pub required_features: Order,
}

/// The bench settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct Bench {
    #[derivative(Default(value = "TARGET.clone()"))]
    pub order: OrderKind,
    pub crate_type: Order,
    pub required_features: Order,
}

// Dependency tables:

/// The dependencies settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields)]
pub struct Dependencies {
    pub order: OrderKind,
    #[derivative(Default(value = "InlineKind::Manual(Some(1))"))]
    pub inline: InlineKind,
}

/// The dev-dependencies settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields)]
pub struct DevDependencies {
    pub order: OrderKind,
    #[derivative(Default(value = "InlineKind::Manual(Some(1))"))]
    pub inline: InlineKind,
}

/// The build-dependencies settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields)]
pub struct BuildDependencies {
    pub order: OrderKind,
    #[derivative(Default(value = "InlineKind::Manual(Some(1))"))]
    pub inline: InlineKind,
}

/// The target settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields)]
pub struct Target {
    pub order: OrderKind,
    #[derivative(Default(value = "InlineKind::Manual(None)"))]
    pub inline: InlineKind,
}

/// The badges settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields)]
pub struct Badges {
    pub order: OrderKind,
    #[derivative(Default(value = "InlineKind::Manual(Some(1))"))]
    pub inline: InlineKind,
    #[serde(rename = "*")]
    pub children: Order,
}

/// The features settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields)]
pub struct Features {
    pub order: OrderKind,
    #[serde(rename = "*")]
    pub children: Order,
}

/// The patch settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields)]
pub struct Patch {
    pub order: OrderKind,
}

/// The replace settings (deprecated).
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields)]
pub struct Replace {
    pub order: OrderKind,
}

/// The profile settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields)]
pub struct Profile {
    pub order: OrderKind,
}

/// The workspace settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields)]
pub struct Workspace {
    pub order: OrderKind,
    pub members: Order,
    pub exclude: Order,
}

/// A general setting with exactly one action: order.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields)]
pub struct Order {
    pub order: OrderKind,
}

#[cfg(test)]
mod test {
    use super::{InlineKind, Settings};
    use anyhow::Result;

    #[test]
    fn empty() -> Result<()> {
        const DEFAULT_CONFIG: &str = include_str!("../assets/cargofmt.default.toml");
        const EMPTY_CONFIG: &str = include_str!("../assets/cargofmt.empty.toml");
        let settings: Settings = toml::from_str(EMPTY_CONFIG)?;
        let config = toml::to_string(&settings)?;
        assert_eq!(DEFAULT_CONFIG, config);
        Ok(())
    }

    #[test]
    fn full() -> Result<()> {
        const CONFIG: &str = include_str!("../assets/cargofmt.full.toml");
        let settings: Settings = toml::from_str(CONFIG)?;
        let config = toml::to_string(&settings)?;
        assert_eq!(CONFIG, config);
        Ok(())
    }

    #[test]
    fn inline_auto() -> Result<()> {
        const CONFIG: &str = "
            [package]\n\
            inline = \"Auto\"
        ";
        let settings: Settings = toml::from_str(CONFIG)?;
        assert_eq!(InlineKind::Auto, settings.package.inline);
        Ok(())
    }

    #[test]
    fn inline_never() -> Result<()> {
        const CONFIG: &str = "
            [package]\n\
            inline = \"Never\"
        ";
        let settings: Settings = toml::from_str(CONFIG)?;
        assert_eq!(InlineKind::Manual(None), settings.package.inline);
        Ok(())
    }

    #[test]
    fn inline_always() -> Result<()> {
        const CONFIG: &str = "
            [package]\n\
            inline = \"Always\"
        ";
        let settings: Settings = toml::from_str(CONFIG)?;
        assert_eq!(InlineKind::Manual(Some(0)), settings.package.inline);
        Ok(())
    }

    #[test]
    fn inline_level_1() -> Result<()> {
        const CONFIG: &str = "
            [package]\n\
            inline = 1
        ";
        let settings: Settings = toml::from_str(CONFIG)?;
        assert_eq!(InlineKind::Manual(Some(1)), settings.package.inline);
        Ok(())
    }

    #[test]
    fn inline_level_9() -> Result<()> {
        const CONFIG: &str = "
            [package]\n\
            inline = 9
        ";
        let settings: Settings = toml::from_str(CONFIG)?;
        assert_eq!(InlineKind::Manual(Some(9)), settings.package.inline);
        Ok(())
    }
}
