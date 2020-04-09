//! Dependency tables (union of sections).

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct DependencyTables {
    pub(super) dependencies: IndexMap<String, Dependency>,
    pub(super) dev_dependencies: IndexMap<String, Dependency>,
    pub(super) build_dependencies: IndexMap<String, Dependency>,
    pub(super) target: Target,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Target {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Dependency {
    pub version: Option<String>,
    pub registry: Option<String>,
    pub registry_index: Option<String>,
    pub path: Option<String>,
    pub git: Option<String>,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub rev: Option<String>,
    pub features: Vec<String>,
    pub optional: bool,
    pub default_features: Option<bool>,
    pub package: Option<String>,
}
