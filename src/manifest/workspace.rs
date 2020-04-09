use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct Workspace {
    pub members: Vec<String>,
    pub default_members: Vec<String>,
    pub exclude: Vec<String>,
}
