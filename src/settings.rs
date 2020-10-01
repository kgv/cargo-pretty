#[doc(inline)]
pub use self::{
    badges::Badges,
    dependency_tables::{BuildDependencies, Dependencies, DevDependencies, Targets},
    features::Features,
    package::Package,
    patch::Patch,
    profile::Profiles,
    replace::Replace,
    target_tables::{Bench, Bin, Example, Lib, Test},
    workspace::Workspace,
};

use crate::{inline::Inline, order::Order};
use derivative::Derivative;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_diff::SerdeDiff;
use std::iter::FromIterator;

lazy_static! {
    static ref MANIFEST: Order = Order::from_iter(vec![
        "package",
        "lib",
        "bin",
        "example",
        "test",
        "bench",
        "dependencies",
        "dev-dependencies",
        "build-dependencies",
        "target",
        "badges",
        "features",
        "replace",
        "patch",
        "replace",
        "profile",
        "workspace",
    ]);
    static ref PACKAGE: Order = Order::from_iter(vec![
        "name",
        "version",
        "authors",
        "edition",
        "description",
        "documentation",
        "readme",
        "homepage",
        "repository",
        "license",
        "license-file",
        "keywords",
        "categories",
        "workspace",
        "build",
        "links",
        "exclude",
        "include",
        "publish",
        "metadata",
        "default-run",
        "autobins",
        "autoexamples",
        "autotests",
        "autobenches",
    ]);
    static ref TARGETS: Order = Order::from_iter(vec!["lib", "bin", "example", "test", "bench"]);
    static ref TARGET: Order = Order::from_iter(vec![
        "name",
        "path",
        "test",
        "doctest",
        "bench",
        "doc",
        "plugin",
        "proc-macro",
        "harness",
        "edition",
        "crate-type",
        "required-features",
    ]);
    static ref DEPENDENCIES: Order = Order::from_iter(vec![
        "dependencies",
        "dev-dependencies",
        "build-dependencies",
    ]);
    static ref DEPENDENCY: Order = Order::from_iter(vec![
        "version",
        "git",
        "branch",
        "rev",
        "tag",
        "path",
        "registry",
        "package",
        "optional",
        "default-features",
        "features",
    ]);
    static ref BADGES: Order = Order::from_iter(vec![
        "appveyor",
        "circle-ci",
        "cirrus-ci",
        "gitlab",
        "azure-devops",
        "travis-ci",
        "bitbucket-pipelines",
        "codecov",
        "coveralls",
        "is-it-maintained-issue-resolution",
        "is-it-maintained-open-issues",
        "maintenance",
    ]);
    static ref BADGE: Order = Order::from_iter(vec![
        "repository",
        "branch",
        "service",
        "project",
        "pipeline",
        "build",
    ]);
    static ref FEATURES: Order = Order::from_iter(vec!["default"]);
    static ref PROFILES: Order = Order::from_iter(vec!["dev", "release", "test", "bench"]);
    static ref PROFILE: Order = Order::from_iter(vec![
        "opt-level",
        "debug",
        "debug-assertions",
        "overflow-checks",
        "lto",
        "panic",
        "incremental",
        "codegen-units",
        "rpath",
    ]);
    static ref WORKSPACE: Order = Order::from_iter(vec!["members", "default-members", "exclude"]);
}

/// Settings.
#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct Settings {
    #[derivative(Default(value = "MANIFEST.clone()"))]
    pub order: Order,
    pub package: Package,
    pub lib: Lib,
    pub bin: Bin,
    pub example: Example,
    pub test: Test,
    pub bench: Bench,
    pub dependencies: Dependencies,
    pub dev_dependencies: DevDependencies,
    pub build_dependencies: BuildDependencies,
    pub targets: Targets,
    pub badges: Badges,
    pub features: Features,
    pub patch: Patch,
    pub replace: Replace,
    pub profiles: Profiles,
    pub workspace: Workspace,
}

pub mod package {
    use super::*;

    /// The package settings.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Package {
        #[derivative(Default(value = "PACKAGE.clone()"))]
        pub order: Order,
        #[derivative(Default(value = "Inline::Manual(Some(1))"))]
        pub inline: Inline,
        pub authors: Authors,
        pub keywords: Keywords,
        pub categories: Categories,
        pub exclude: Exclude,
        pub include: Include,
        pub metadata: Metadata,
    }

    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Authors {
        pub order: Order,
    }

    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Keywords {
        pub order: Order,
    }

    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Categories {
        pub order: Order,
    }

    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Exclude {
        pub order: Order,
    }

    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Include {
        pub order: Order,
    }

    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Metadata {
        pub order: Order,
        #[derivative(Default(value = "Inline::Manual(None)"))]
        pub inline: Inline,
    }
}

// Target tables.
pub mod target_tables {
    use super::*;

    /// The lib settings.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
    pub struct Lib {
        #[derivative(Default(value = "TARGET.clone()"))]
        pub order: Order,
        pub crate_type: CrateType,
    }

    /// The bin settings.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
    pub struct Bin {
        #[derivative(Default(value = "TARGET.clone()"))]
        pub order: Order,
        pub required_features: RequiredFeatures,
    }

    /// The example settings.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
    pub struct Example {
        #[derivative(Default(value = "TARGET.clone()"))]
        pub order: Order,
        pub required_features: RequiredFeatures,
    }

    /// The test settings.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
    pub struct Test {
        #[derivative(Default(value = "TARGET.clone()"))]
        pub order: Order,
        pub required_features: RequiredFeatures,
    }

    /// The bench settings.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
    pub struct Bench {
        #[derivative(Default(value = "TARGET.clone()"))]
        pub order: Order,
        pub crate_type: CrateType,
        pub required_features: RequiredFeatures,
    }

    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct CrateType {
        pub order: Order,
    }

    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct RequiredFeatures {
        pub order: Order,
    }
}

// Dependency tables.
pub mod dependency_tables {
    use super::*;

    /// The dependencies settings.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Dependencies {
        pub order: Order,
        #[derivative(Default(value = "Inline::Manual(Some(1))"))]
        pub inline: Inline,
        #[serde(rename = "*")]
        pub dependency: Dependency,
    }

    /// The dev-dependencies settings.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct DevDependencies {
        pub order: Order,
        #[derivative(Default(value = "Inline::Manual(Some(1))"))]
        pub inline: Inline,
        #[serde(rename = "*")]
        pub dependency: Dependency,
    }

    /// The build-dependencies settings.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct BuildDependencies {
        pub order: Order,
        #[derivative(Default(value = "Inline::Manual(Some(1))"))]
        pub inline: Inline,
        #[serde(rename = "*")]
        pub dependency: Dependency,
    }

    /// The target settings.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Targets {
        pub order: Order,
        pub dependencies: Dependencies,
        pub dev_dependencies: DevDependencies,
        pub build_dependencies: BuildDependencies,
        #[serde(rename = "*")]
        pub target: Target,
    }

    /// Any target.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Target {
        #[derivative(Default(value = "DEPENDENCIES.clone()"))]
        pub order: Order,
    }

    /// Any dependency.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Dependency {
        #[derivative(Default(value = "DEPENDENCY.clone()"))]
        pub order: Order,
    }
}

pub mod badges {
    use super::*;

    /// The badges settings.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Badges {
        #[derivative(Default(value = "BADGES.clone()"))]
        pub order: Order,
        #[derivative(Default(value = "Inline::Manual(Some(1))"))]
        pub inline: Inline,
        #[serde(rename = "*")]
        pub badge: Badge,
    }

    /// Any badge.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Badge {
        #[derivative(Default(value = "BADGE.clone()"))]
        pub order: Order,
    }
}

pub mod features {
    use super::*;

    /// The features settings.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Features {
        #[derivative(Default(value = "FEATURES.clone()"))]
        pub order: Order,
        #[serde(rename = "*")]
        pub feature: Feature,
    }

    /// Any feature.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Feature {
        pub order: Order,
    }
}

pub mod patch {
    use super::*;

    /// The patch settings.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Patch {
        pub order: Order,
        #[derivative(Default(value = "Inline::Manual(Some(2))"))]
        pub inline: Inline,
    }
}

pub mod replace {
    use super::*;

    /// The replace settings (deprecated).
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Replace {
        pub order: Order,
    }
}

pub mod profile {
    use super::*;

    /// The profile settings.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Profiles {
        #[derivative(Default(value = "PROFILES.clone()"))]
        pub order: Order,
        #[derivative(Default(value = "Inline::Manual(Some(1))"))]
        pub inline: Inline,
        #[serde(rename = "*")]
        pub profile: Profile,
    }

    /// Any profile.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Profile {
        #[derivative(Default(value = "PROFILE.clone()"))]
        pub order: Order,
    }
}

pub mod workspace {
    use super::*;

    /// The workspace settings.
    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Workspace {
        #[derivative(Default(value = "WORKSPACE.clone()"))]
        pub order: Order,
        pub members: Members,
        pub default_members: DefaultMembers,
        pub exclude: Exclude,
    }

    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Members {
        pub order: Order,
    }

    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct DefaultMembers {
        pub order: Order,
    }

    #[derive(Clone, Debug, Derivative, Deserialize, PartialEq, SerdeDiff, Serialize)]
    #[derivative(Default)]
    #[serde(default, deny_unknown_fields)]
    pub struct Exclude {
        pub order: Order,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Ordered;
    use anyhow::Result;

    #[test]
    fn default() -> Result<()> {
        const DEFAULT_CONFIG: &str = include_str!("../manifestfmt.default.toml");
        let default: Settings = toml::from_str(DEFAULT_CONFIG)?;
        assert_eq!(default, Settings::default());
        Ok(())
    }

    #[test]
    fn empty() -> Result<()> {
        const DEFAULT_CONFIG: &str = include_str!("../manifestfmt.default.toml");
        const EMPTY_CONFIG: &str = "";
        let empty: Settings = toml::from_str(EMPTY_CONFIG)?;
        let default: Settings = toml::from_str(DEFAULT_CONFIG)?;
        assert_eq!(empty, default);
        Ok(())
    }

    #[test]
    fn order() -> Result<()> {
        const CONFIG: &str = r#"
            [package]
            order = "Alphabetic"
        "#;
        let settings: Settings = toml::from_str(CONFIG)?;
        assert_eq!(settings.package.order, Order::Ordered(Ordered::Alphabetic));
        Ok(())
    }

    #[test]
    fn inline_auto() -> Result<()> {
        const CONFIG: &str = r#"
            [package]
            inline = "Auto"
        "#;
        let settings: Settings = toml::from_str(CONFIG)?;
        assert_eq!(settings.package.inline, Inline::Auto);
        Ok(())
    }

    #[test]
    fn inline_none() -> Result<()> {
        const CONFIG: &str = r#"
            [package]
            inline = "None"
        "#;
        let settings: Settings = toml::from_str(CONFIG)?;
        assert_eq!(settings.package.inline, Inline::Manual(None));
        Ok(())
    }

    #[test]
    fn inline_level_0() -> Result<()> {
        const CONFIG: &str = r#"
            [package]
            inline = 0
        "#;
        let settings: Settings = toml::from_str(CONFIG)?;
        assert_eq!(settings.package.inline, Inline::Manual(Some(0)));
        Ok(())
    }

    #[test]
    fn inline_level_1() -> Result<()> {
        const CONFIG: &str = r#"
            [package]
            inline = 1
        "#;
        let settings: Settings = toml::from_str(CONFIG)?;
        assert_eq!(settings.package.inline, Inline::Manual(Some(1)));
        Ok(())
    }

    #[test]
    fn inline_level_9() -> Result<()> {
        const CONFIG: &str = r#"
            [package]
            inline = 9
        "#;
        let settings: Settings = toml::from_str(CONFIG)?;
        assert_eq!(settings.package.inline, Inline::Manual(Some(9)));
        Ok(())
    }
}
