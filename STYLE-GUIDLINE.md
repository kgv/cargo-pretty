# Cargo manifest style guideline

- Limit lines to 100 characters.
- When using a multi-line block, put the opening and closing braces or brackets
  on their own lines, and indent with four spaces.
- prefer use double quotes instead of single quotes,  
  TODO: Why cargo creates tample with single quotes for the `package.edition`
  and double quotes for the rest?

## Sections

Preference:

- preferred order for sections:
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

### Package section

Preference:

- preferred order for the `package` section:
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
- prefer start with a capital letter and end with a dot for
  `package.description`.

### Dependencies tables section

Preference:

- preferred order for dependencies tables section
  1. `dependencies`,
  2. `dev-dependencies`,
  3. `build-dependencies`,
  4. `target`;
- order for each dependency table: alphabetical,

TODO:

- preferred order for `dependencies`, `dev-dependencies`, `build-dependencies`:
  - version (the first key),
  - other keys are in alphabetical order,
  - features (the last key).

### Target tables section

Preference:

- preferred order for target tables section:
  1. `lib`,
  2. `bin`,
  3. `example`,
  4. `test`,
  5. `bench`;
- preferred order for each target table:
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

### Badges section

Preference:

- preferred order for the `badges` section: alphabetical,
- inline level for the `badges` section: 1.

### Features section

Requirements:

- features should never be negative (e.g., foo is good, no-foo is bad),
- features should be named using kebab-case.

Preference:

- preferred order for the `features` section: alphabetical,
- inline level for the `badges` section: 1,
- prefer short but descriptive feature names,
- prefer not to have default features.

## Comments

```toml
# This is comment 0 for `a`.
[a]
# This is comment 0 for `a.b`.

# This is comment 1 for `a.b`.
    # This is comment 2 for `a.b`.
b = "cargofmt" # This is comment 3 for `a.b`.
c = "0.1.0" # This is comment 0 for `a.c`.
```

Preference:

- prefer to use "full-line" instead of "end of a line" outer comment.

## Links

- [0](https://github.com/pingcap/style-guide/blob/master/docs/rust/modules.md#cargotoml)
- [1](https://github.com/killercup/cargo-edit/issues/361)
