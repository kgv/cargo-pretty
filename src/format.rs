use crate::{settings::Settings, sort::Sort};
use optional_index::OptionalIndexMut;
use std::fmt::Display;
use toml_lalrpop::{
    format::Independent,
    value::{Item, Table},
};

// Array of tables iterator.
fn array_of_tables(optional_item: Option<&mut Item>) -> impl Iterator<Item = &mut Item> {
    optional_item
        .and_then(|item| item.as_array_mut())
        .map(|array| array.iter_mut())
        .into_iter()
        .flatten()
}

// Table iterator.
fn table(optional_item: Option<&mut Item>) -> impl Iterator<Item = (&String, &mut Item)> {
    optional_item
        .and_then(|item| item.as_table_mut())
        .map(|table| table.iter_mut())
        .into_iter()
        .flatten()
}

/// Format.
pub trait Format {
    fn format<'a>(&'a mut self, settings: &'a Settings) -> Box<dyn 'a + Display> {
        self.sort(settings);
        self.inline(settings)
    }

    fn sort(&mut self, settings: &Settings);

    fn inline<'a>(&'a mut self, settings: &'a Settings) -> Box<dyn 'a + Display>;
}

impl Format for Table {
    fn sort(&mut self, settings: &Settings) {
        Sort::sort(self, &settings.order);
        // package.
        let mut package = self.optional_index_mut("package");
        package.sort(&settings.package.order);
        package
            .optional_index_mut("authors")
            .sort(&settings.package.authors.order);
        package
            .optional_index_mut("keywords")
            .sort(&settings.package.keywords.order);
        package
            .optional_index_mut("categories")
            .sort(&settings.package.categories.order);
        package
            .optional_index_mut("exclude")
            .sort(&settings.package.exclude.order);
        package
            .optional_index_mut("include")
            .sort(&settings.package.include.order);
        // Target tables:
        {
            // lib.
            let mut lib = self.optional_index_mut("lib");
            lib.sort(&settings.lib.order);
            lib.optional_index_mut("crate-type")
                .sort(&settings.lib.crate_type.order);
            // bin.
            for bin in array_of_tables(self.optional_index_mut("bin")) {
                bin.sort(&settings.bin.order);
                bin.optional_index_mut("required-features")
                    .sort(&settings.bin.required_features.order);
            }
            // example.
            for example in array_of_tables(self.optional_index_mut("example")) {
                example.sort(&settings.example.order);
                example
                    .optional_index_mut("required-features")
                    .sort(&settings.example.required_features.order);
            }
            // test.
            for test in array_of_tables(self.optional_index_mut("test")) {
                test.sort(&settings.test.order);
                test.optional_index_mut("required-features")
                    .sort(&settings.test.required_features.order);
            }
            // bench.
            for bench in array_of_tables(self.optional_index_mut("bench")) {
                bench.sort(&settings.bench.order);
                bench
                    .optional_index_mut("crate-type")
                    .sort(&settings.bench.crate_type.order);
                bench
                    .optional_index_mut("required-features")
                    .sort(&settings.bench.required_features.order);
            }
        }
        // Dependency tables:
        {
            // dependencies.
            let mut dependencies = self.optional_index_mut("dependencies");
            dependencies.sort(&settings.dependencies.order);
            for (_, dependency) in table(dependencies) {
                dependency.sort(&settings.dependencies.dependency.order);
            }
            // dev-dependencies.
            let mut dev_dependencies = self.optional_index_mut("dev-dependencies");
            dev_dependencies.sort(&settings.dev_dependencies.order);
            for (_, dependency) in table(dev_dependencies) {
                dependency.sort(&settings.dev_dependencies.dependency.order);
            }
            // build-dependencies.
            let mut build_dependencies = self.optional_index_mut("build-dependencies");
            build_dependencies.sort(&settings.build_dependencies.order);
            for (_, dependency) in table(build_dependencies) {
                dependency.sort(&settings.build_dependencies.dependency.order);
            }
            // target.
            let mut targets = self.optional_index_mut("target");
            targets.sort(&settings.targets.order);
            for (_, target) in table(targets) {
                target.sort(&settings.targets.target.order);
            }
        }
        // badges.
        let mut badges = self.optional_index_mut("badges");
        badges.sort(&settings.badges.order);
        for (_, badge) in table(badges) {
            badge.sort(&settings.badges.badge.order);
        }
        // features.
        let mut features = self.optional_index_mut("features");
        features.sort(&settings.features.order);
        for (_, feature) in table(features) {
            feature.sort(&settings.features.feature.order);
        }
        // patch.
        self.optional_index_mut("patch").sort(&settings.patch.order);
        // replace.
        self.optional_index_mut("replace")
            .sort(&settings.replace.order);
        // profile.
        let mut profiles = self.optional_index_mut("profile");
        profiles.sort(&settings.profiles.order);
        for (_, profile) in table(profiles) {
            profile.sort(&settings.profiles.profile.order);
        }
        // workspace.
        let mut workspace = self.optional_index_mut("workspace");
        workspace.sort(&settings.workspace.order);
        workspace
            .optional_index_mut("members")
            .sort(&settings.workspace.members.order);
        workspace
            .optional_index_mut("default-members")
            .sort(&settings.workspace.default_members.order);
        workspace
            .optional_index_mut("exclude")
            .sort(&settings.workspace.exclude.order);
    }

    fn inline<'a>(&'a mut self, settings: &'a Settings) -> Box<dyn 'a + Display> {
        let is_inline = move |key: &[&str]| match key {
            ["profile", rest @ ..] => settings.profiles.inline.level(rest.len()).is_inline(),
            ["patch", rest @ ..] => settings.patch.inline.level(rest.len()).is_inline(),
            ["badges", rest @ ..] => settings.badges.inline.level(rest.len()).is_inline(),
            ["target", _, "build-dependencies", rest @ ..] => settings
                .targets
                .build_dependencies
                .inline
                .level(rest.len())
                .is_inline(),
            ["target", _, "dev-dependencies", rest @ ..] => settings
                .targets
                .dev_dependencies
                .inline
                .level(rest.len())
                .is_inline(),
            ["target", _, "dependencies", rest @ ..] => settings
                .targets
                .dependencies
                .inline
                .level(rest.len())
                .is_inline(),
            ["build-dependencies", rest @ ..] => settings
                .build_dependencies
                .inline
                .level(rest.len())
                .is_inline(),
            ["dev-dependencies", rest @ ..] => settings
                .dev_dependencies
                .inline
                .level(rest.len())
                .is_inline(),
            ["dependencies", rest @ ..] => {
                settings.dependencies.inline.level(rest.len()).is_inline()
            }
            ["package", "metadata", rest @ ..] => settings
                .package
                .metadata
                .inline
                .level(rest.len())
                .is_inline(),
            ["package", rest @ ..] => settings.package.inline.level(rest.len()).is_inline(),
            _ => false,
        };
        Box::new(Independent::new(self, is_inline))
    }
}
