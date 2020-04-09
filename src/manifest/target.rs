use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Library(Target);

#[derive(Debug, Deserialize, Serialize)]
pub struct Binary(Target);

#[derive(Debug, Deserialize, Serialize)]
pub struct Example(Target);

#[derive(Debug, Deserialize, Serialize)]
pub struct Test(Target);

#[derive(Debug, Deserialize, Serialize)]
pub struct Benchmark(Target);

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
struct Target {
    name: String,
    path: String,
    test: bool,
    doctest: bool,
    bench: bool,
    doc: bool,
    plugin: bool,
    proc_macro: bool,
    harness: bool,
    edition: String,
    crate_type: Vec<String>,
    required_features: Vec<String>,
}
