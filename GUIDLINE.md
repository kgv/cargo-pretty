# Cargo manifest format guideline

- Sections should be separated by a single blank line.
- Разделитель ключ-значение должен обрамляться одиночными пробелами.
- Limit lines to 100 characters.
- When using a multi-line block, put the opening and closing braces or brackets
  on their own lines, and indent with four spaces.

## Root

- Order for `root` section:
  - `package`,
  - `lib`,
  - `bin`,
  - `example`,
  - `test`,
  - `bench`,
  - `dependencies`,
  - `dev-dependencies`,
  - `build-dependencies`,
  - `target`,
  - `badges`,
  - `features`,
  - `replace`,
  - `patch`,
  - `profile`,
  - `workspace`;

## Package

- Order for the `package` section:
  - `name`,
  - `version`,
  - `authors`,
  - `edition`,
  - `description`,
  - `documentation`,
  - `readme`,
  - `homepage`,
  - `repository`,
  - `license`,
  - `license-file`,
  - `keywords`,
  - `categories`,
  - `workspace`,
  - `build`,
  - `links`,
  - `exclude`,
  - `include`,
  - `publish`,
  - `metadata`,
  - `default-run`,
  - `autobins`,
  - `autoexamples`,
  - `autotests`,
  - `autobenches`;

## Dependencies tables

- Order for all dependency tables (`dependencies`, `dev-dependencies`,
  `build-dependencies`, `target`): alphabetical,

## Target tables

- Order for all target tables (`lib`, `bin`, `example`, `test`, `bench`):
  - `name`,
  - `path`,
  - `test`,
  - `doctest`,
  - `bench`,
  - `doc`,
  - `plugin`,
  - `proc-macro`,
  - `harness`,
  - `edition`,
  - `crate-type`,
  - `required-features`,

## Features

- Order for features: alphabetical,
- Features should never be negative (e.g., foo is good, no-foo is bad).
- Features should be named using kebab-case.
- Prefer short but descriptive feature names.
- Prefer not to have default features.

## Rest

- `package.description`
  - starts with a capital letter,
  - ends with a point;
- `package.edition`
  - uses single quotes instead of double quotes; TODO: WHY?
- `dependencies`
  - version is the first key
  - features is the last key
  - other keys are in "Forward" order

## Links

[0](https://github.com/pingcap/style-guide/blob/master/docs/rust/modules.md#cargotoml)
[1](https://github.com/killercup/cargo-edit/issues/361)
