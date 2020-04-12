# Cargo manifest format guideline

- Limit lines to 100 characters.
- When using a multi-line block, put the opening and closing braces or brackets
  on their own lines, and indent with four spaces.
- prefer use double quotes instead of single quotes,  
  TODO: Why cargo creates tample with single quotes for the `package.edition`
  and double quotes for the rest?

## Data

- order sections for manifest:
  1. `package`,
  2. `lib`,
  3. `bin`,
  4. `example`,
  5. `test`,
  6. `bench`,
  7. `dependencies`,
  8. `dev-dependencies`,
  9. `build-dependencies`,
  10. `target`,
  11. `badges`,
  12. `features`,
  13. `replace`,
  14. `patch`,
  15. `profile`,
  16. `workspace`;
- sections should be separated by a single blank line.

### Package

- order for `package`:
  1. `name`,
  2. `version`,
  3. `authors`,
  4. `edition`,
  5. `description`,
  6. `documentation`,
  7. `readme`,
  8. `homepage`,
  9. `repository`,
  10. `license`,
  11. `license-file`,
  12. `keywords`,
  13. `categories`,
  14. `workspace`,
  15. `build`,
  16. `links`,
  17. `exclude`,
  18. `include`,
  19. `publish`,
  20. `metadata`,
  21. `default-run`,
  22. `autobins`,
  23. `autoexamples`,
  24. `autotests`,
  25. `autobenches`;
- `description` is single sentence,
- `description` starts with a capital letter and ends with a dot.

### Target tables

- order for target tables:
  1. `lib`,
  2. `bin`,
  3. `example`,
  4. `test`,
  5. `bench`;
- order for each target table:
  1. `name`,
  2. `path`,
  3. `test`,
  4. `doctest`,
  5. `bench`,
  6. `doc`,
  7. `plugin`,
  8. `proc-macro`,
  9. `harness`,
  10. `edition`,
  11. `crate-type`,
  12. `required-features`.

### Dependencies tables

- order for dependencies tables:
  1. `dependencies`,
  2. `dev-dependencies`,
  3. `build-dependencies`,
  4. `target`;
- order for each dependency table:
  1. `version`,
  2. `git` or `path` or `registry`,
  3. `branch` or `rev` or `tag`,
  4. `package`,
  5. `optional`,
  6. `default-features`,
  7. `features`.

### Badges

- order for `badges`:
  1. `appveyor`,
  2. `circle-ci`,
  3. `cirrus-ci`,
  4. `gitlab`,
  5. `azure-devops`,
  6. `travis-ci`,
  7. `bitbucket-pipelines`,
  8. `codecov`,
  9. `coveralls`,
  10. `is-it-maintained-issue-resolution`,
  11. `is-it-maintained-open-issues`,
  12. `maintenance`;
- order for each badge:
  1. `repository` or `project`,
  2. `branch` or `pipeline`,
  3. `service` or `build`;
- inline level for `badges`: 1.

### Features

- order for `features`: `Alphabetic`,
  1. `default`;
- inline level for `features`: 1,
- features should never be negative (e.g., foo is good, no-foo is bad),
- features should be named using kebab-case,
- prefer short but descriptive feature names,
- prefer not to have default features.

### Patch

### Replace

### Profile

- order for `profile`:
  - `dev`,
  - `release`,
  - `test`,
  - `bench`;
- order for each entry in `profile`:
  - `opt-level`,
  - `debug`,
  - `debug-assertions`,
  - `overflow-checks`,
  - `lto`,
  - `panic`,
  - `incremental`,
  - `codegen-units`,
  - `rpath`;

### Workspace

- order for `workspace`:
  - `members`,
  - `default-members`,
  - `exclude`;
- order for each entry in `workspace`: `Alphabetic`.

## Meta (Comments)

```toml
# This is comment 0 for `a`.
[a]
# This is comment 0 for `a.b`.

# This is comment 1 for `a.b`.
    # This is comment 2 for `a.b`.
b = "cargofmt" # This is comment 3 for `a.b`.
c = "0.1.0" # This is comment 0 for `a.c`.
```

- prefer to use "full-line" instead of "end of a line" comment.

## Links

- [0](https://github.com/pingcap/style-guide/blob/master/docs/rust/modules.md#cargotoml)
- [1](https://github.com/killercup/cargo-edit/issues/361)
