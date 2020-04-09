//! Target tables (union of sections).

use super::Edition;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TargetTables {
    pub(super) lib: Option<Lib>,
    pub(super) bins: Vec<Bin>,
    pub(super) examples: Vec<Example>,
    pub(super) tests: Vec<Test>,
    pub(super) benchs: Vec<Bench>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Target {
    Lib(Lib),
    Bins(Bin),
    Examples(Example),
    Tests(Test),
    Benchs(Bench),
}

#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(rename_all = "kebab-case")]
pub struct Lib {
    /// See [more](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-harness-field).
    pub name: Option<String>,
    pub path: Option<String>,
    #[derivative(Default(value = "true"))]
    pub test: bool,
    #[derivative(Default(value = "true"))]
    pub doctest: bool,
    #[derivative(Default(value = "true"))]
    pub bench: bool,
    #[derivative(Default(value = "true"))]
    pub doc: bool,
    pub plugin: bool,
    pub proc_macro: bool,
    #[derivative(Default(value = "true"))]
    pub harness: bool,
    pub edition: Option<Edition>,
    pub crate_type: Vec<CrateType>,
}

#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(rename_all = "kebab-case")]
pub struct Bin {
    pub name: String,
    pub path: Option<String>,
    #[derivative(Default(value = "true"))]
    pub test: bool,
    #[derivative(Default(value = "true"))]
    pub bench: bool,
    #[derivative(Default(value = "true"))]
    pub doc: bool,
    pub plugin: bool,
    #[derivative(Default(value = "true"))]
    pub harness: bool,
    pub edition: Option<Edition>,
    pub required_features: Vec<String>,
}

#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(rename_all = "kebab-case")]
pub struct Example {
    pub name: String,
    pub path: Option<String>,
    pub test: bool,
    pub bench: bool,
    pub doc: bool,
    pub plugin: bool,
    #[derivative(Default(value = "true"))]
    pub harness: bool,
    pub edition: Option<Edition>,
    pub crate_type: Vec<CrateType>,
    pub required_features: Vec<String>,
}

#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(rename_all = "kebab-case")]
pub struct Test {
    pub name: String,
    pub path: Option<String>,
    #[derivative(Default(value = "true"))]
    pub test: bool,
    pub bench: bool,
    pub doc: bool,
    pub plugin: bool,
    #[derivative(Default(value = "true"))]
    pub harness: bool,
    pub edition: Option<Edition>,
    pub required_features: Vec<String>,
}

#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(rename_all = "kebab-case")]
pub struct Bench {
    pub name: String,
    pub path: Option<String>,
    pub test: bool,
    #[derivative(Default(value = "true"))]
    pub bench: bool,
    pub doc: bool,
    pub plugin: bool,
    #[derivative(Default(value = "true"))]
    pub harness: bool,
    pub edition: Option<Edition>,
    pub required_features: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum CrateType {
    Lib,
    ProcMacro,
    Bin,
}
