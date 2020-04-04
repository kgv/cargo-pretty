//! Cargofmt

#![feature(exclusive_range_pattern)]
#![feature(half_open_range_patterns)]

use self::{format::Format, inline::Inline, order::Order, settings::Settings};
use anyhow::Result;
use log::{debug, trace};
use std::fs::{read_to_string, rename, write};
use toml_edit::{Document, Item, Table};

// rustfmt --backup --config-path=.\\ .\\.rs.bk\\sandbox\\Cargo.toml

// set RUST_LOG=debug
// cargo run -- .\\.rs.bk\\sandbox\\Cargo.toml
// cargo run -- --backup .\\.rs.bk\\sandbox\\Cargo.toml
// cargo run -- --config-path=.\\ .\\.rs.bk\\sandbox\\Cargo.toml
// cargo run -- --backup --config-path=.\\ .\\.rs.bk\\sandbox\\Cargo.toml
fn main() -> Result<()> {
    env_logger::init();

    let options = options::parse();
    trace!("options: {:?}", options);
    let config_file = options.config_path.join("cargofmt.toml");
    debug!("config_file: {}", config_file.display());
    let settings: Settings = toml::from_str(&read_to_string(config_file)?)?;
    trace!("settings: {:?}", settings);
    // let settings = match print_config {
    //     PrintConfig::Default => &*settings::DEFAULT,
    //     PrintConfig::Current => &settings,
    // };

    for manifest_file in &options.files {
        debug!("manifest_file: {}", manifest_file.display());
        let mut manifest = read_to_string(manifest_file)?.parse::<Document>()?;
        format_manifest(&mut manifest, &settings)?;
        // println!("{}", toml::to_string(&manifest)?);
        if options.backup {
            let backup_file = manifest_file.with_extension("toml.bk");
            rename(manifest_file, backup_file)?;
        }
        write(manifest_file, manifest.to_string().trim_start())?;
    }

    Ok(())
}

// fn temp(from: &Item, to: &mut InlineTable) {
//     use toml_edit::{Document, InlineTable, Item, Key, Table};
//     if let Some(table_like) = from.as_table_like() {
//         let iter = table_like.iter().map(|(key, value)| match value {
//             Item::Value(value) => (key.parse::<Key>().unwrap(), 5),
//             // Item::Table(table) => {
//             //     let mut value = InlineTable::default();
//             //     table_to_inline_table(table, &mut value);
//             //     (key, Value::InlineTable(value))
//             // }
//             _ => unimplemented!(),
//         });
//         use std::iter::FromIterator;
//         // let value: Value = toml_edit::Value::from_iter(vec![
//         //     ("c".parse::<Key>().unwrap(), 4),
//         //     ("d".parse::<Key>().unwrap(), 5),
//         //     ("e".parse::<Key>().unwrap(), 6),
//         // ]);
//         // for (key, item) in table_like.iter() {
//         // }
//     }
// }

fn format_manifest(manifest: &mut Document, settings: &Settings) -> Result<()> {
    let root = manifest.as_table_mut();
    root.order(&settings.order);
    format_package(&mut root["package"], settings);
    format_target_tables(root, settings);
    format_dependency_tables(root, settings);
    format_badges(&mut root["badges"], settings);
    format_features(&mut root["features"], settings);
    format_patch(&mut root["patch"], settings);
    format_replace(&mut root["replace"], settings);
    format_profile(&mut root["profile"], settings);
    format_workspace(&mut root["workspace"], settings);
    Ok(())
}

fn format_package(package: &mut Item, settings: &Settings) {
    trace!("package");
    package.order(&settings.package.order);
    package["authors"].order(&settings.package.authors.order);
    package["keywords"].order(&settings.package.keywords.order);
    package["categories"].order(&settings.package.categories.order);
    package["exclude"].order(&settings.package.exclude.order);
    package["include"].order(&settings.package.include.order);
    package.inline(&settings.package.inline);
    package.format();
}

fn format_target_tables(root: &mut Table, settings: &Settings) {
    format_lib(&mut root["lib"], settings);
    format_bins(&mut root["bin"], settings);
    format_examples(&mut root["example"], settings);
    format_tests(&mut root["test"], settings);
    format_benchs(&mut root["bench"], settings);
}

fn format_lib(lib: &mut Item, settings: &Settings) {
    trace!("lib");
    lib.order(&settings.lib.order);
    lib["crate-type"].order(&settings.lib.crate_type.order);
    lib.format();
}

fn format_bins(bins: &mut Item, settings: &Settings) {
    trace!("bin");
    let len = bins.as_array_of_tables().map_or(0, |v| v.len());
    for index in 0..len {
        bins[index].order(&settings.bin.order);
        bins[index]["required-features"].order(&settings.bin.required_features.order);
        bins[index].format();
    }
}

fn format_examples(examples: &mut Item, settings: &Settings) {
    trace!("example");
    let len = examples.as_array_of_tables().map_or(0, |v| v.len());
    for index in 0..len {
        examples[index].order(&settings.example.order);
        examples[index]["required-features"].order(&settings.example.required_features.order);
        examples[index].format();
    }
}

fn format_tests(tests: &mut Item, settings: &Settings) {
    trace!("test");
    let len = tests.as_array_of_tables().map_or(0, |v| v.len());
    for index in 0..len {
        tests[index].order(&settings.test.order);
        tests[index]["required-features"].order(&settings.test.required_features.order);
        tests[index].format();
    }
}

fn format_benchs(benchs: &mut Item, settings: &Settings) {
    trace!("bench");
    let len = benchs.as_array_of_tables().map_or(0, |v| v.len());
    for index in 0..len {
        benchs[index].order(&settings.bench.order);
        benchs[index]["crate-type"].order(&settings.bench.crate_type.order);
        benchs[index]["required-features"].order(&settings.bench.required_features.order);
        benchs[index].format();
    }
}

fn format_dependency_tables(root: &mut Table, settings: &Settings) {
    format_dependencies(&mut root["dependencies"], settings);
    format_dev_dependencies(&mut root["dev-dependencies"], settings);
    format_build_dependencies(&mut root["build-dependencies"], settings);
    format_target(&mut root["target"], settings);
}

fn format_dependencies(dependencies: &mut Item, settings: &Settings) {
    trace!("dependencies");
    dependencies.order(&settings.dependencies.order);
    dependencies.inline(&settings.dependencies.inline);
    dependencies.format();
}

fn format_dev_dependencies(dev_dependencies: &mut Item, settings: &Settings) {
    trace!("dev-dependencies");
    dev_dependencies.order(&settings.dev_dependencies.order);
    dev_dependencies.inline(&settings.dependencies.inline);
    dev_dependencies.format();
}

fn format_build_dependencies(build_dependencies: &mut Item, settings: &Settings) {
    trace!("dev-dependencies");
    build_dependencies.order(&settings.build_dependencies.order);
    build_dependencies.inline(&settings.build_dependencies.inline);
    build_dependencies.format();
}

fn format_target(target: &mut Item, settings: &Settings) {
    trace!("target");
    target.order(&settings.target.order);
    target.inline(&settings.target.inline);
    target.format();
}

fn format_badges(badges: &mut Item, settings: &Settings) {
    trace!("badges");
    badges.order(&settings.badges.order);
    // match badges {
    //     Item::Table(table)
    // }
    // if let Some(table_like) = badges.as_table_like() {
    //     for badge in table_like.iter() {
    //         badge.order(&settings.badges.children.order);
    //     }
    // }
    // let mut index = 0;
    // while !badges[index].is_none() {
    //     println!("badges[index]: {:?}", badges[index]);
    //     badges[index].order(&settings.badges.children.order);
    //     index += 1;
    // }
    badges.inline(&settings.badges.inline);
    badges.format();
}

fn format_features(features: &mut Item, settings: &Settings) {
    trace!("features");
    features.order(&settings.features.order);
    features.format();
}

fn format_patch(patch: &mut Item, settings: &Settings) {
    trace!("patch");
    patch.order(&settings.patch.order);
    patch.format();
}

fn format_replace(replace: &mut Item, settings: &Settings) {
    trace!("replace");
    replace.order(&settings.replace.order);
    replace.format();
}

fn format_profile(profile: &mut Item, settings: &Settings) {
    trace!("profile");
    profile.order(&settings.profile.order);
    profile.format();
}

fn format_workspace(workspace: &mut Item, settings: &Settings) {
    trace!("workspace");
    workspace.order(&settings.workspace.order);
    workspace.format();
}

mod format;
mod inline;
mod options;
mod order;
mod settings;

#[test]
fn test() {}
