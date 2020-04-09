pub use self::profile::Profile;

use self::{
    badges::Badges,
    dependency_tables::DependencyTables,
    target_tables::{Bench, Bin, Example, Lib, TargetTables, Test},
    workspace::Workspace,
};
use dependency_tables::Dependency;
use derivative::Derivative;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct Manifest {
    package: Option<Package>,
    #[serde(flatten)]
    target_tables: TargetTables,
    #[serde(flatten)]
    dependency_tables: DependencyTables,
    badges: Badges,
    features: IndexMap<String, Vec<String>>,
    patch: IndexMap<String, IndexMap<String, Dependency>>,
    replace: IndexMap<String, IndexMap<String, Dependency>>,
    profile: Option<Profile>,
    workspace: Option<Workspace>,
}

impl Manifest {
    pub fn package(&self) -> Option<&Package> {
        self.package.as_ref()
    }

    pub fn lib(&self) -> Option<&Lib> {
        self.target_tables.lib.as_ref()
    }

    pub fn lib_mut(&mut self) -> Option<&mut Lib> {
        self.target_tables.lib.as_mut()
    }

    pub fn bins(&self) -> &[Bin] {
        &self.target_tables.bins
    }

    pub fn bins_mut(&mut self) -> &mut [Bin] {
        &mut self.target_tables.bins
    }

    pub fn examples(&self) -> &[Example] {
        &self.target_tables.examples
    }

    pub fn examples_mut(&mut self) -> &mut [Example] {
        &mut self.target_tables.examples
    }

    pub fn tests(&self) -> &[Test] {
        &self.target_tables.tests
    }

    pub fn tests_mut(&mut self) -> &mut [Test] {
        &mut self.target_tables.tests
    }

    pub fn benchs(&self) -> &[Bench] {
        &self.target_tables.benchs
    }

    pub fn benchs_mut(&mut self) -> &mut [Bench] {
        &mut self.target_tables.benchs
    }
}

#[derive(Clone, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    edition: Edition,
    description: Option<String>,
    documentation: Option<String>,
    readme: Option<String>,
    homepage: Option<String>,
    repository: Option<String>,
    license: Option<String>,
    license_file: Option<String>,
    keywords: Vec<String>,
    categories: Vec<String>,
    workspace: Option<String>,
    build: Option<String>,
    links: Option<String>,
    exclude: Option<String>,
    include: Option<String>,
    publish: Option<String>,
    metadata: Option<Metadata>,
    default_run: Option<String>,
    #[derivative(Default(value = "true"))]
    autobins: bool,
    #[derivative(Default(value = "true"))]
    autoexamples: bool,
    #[derivative(Default(value = "true"))]
    autotests: bool,
    #[derivative(Default(value = "true"))]
    autobenches: bool,
}

#[derive(Clone, Copy, Debug, Derivative, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[derivative(Default)]
pub enum Edition {
    #[derivative(Default)]
    #[serde(rename = "2015")]
    One,
    #[serde(rename = "2018")]
    Two,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Metadata(String);

mod badges;
mod dependency_tables;
mod features;
mod profile;
mod target_tables;
mod workspace;
