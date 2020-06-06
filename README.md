# Cargofmt

[![Build Status](https://travis-ci.org/kgv/cargofmt.svg?branch=master)](https://travis-ci.org/kgv/cargofmt)
[![Build Status](https://ci.appveyor.com/api/projects/status/github/kgv/cargofmt?svg=true)](https://ci.appveyor.com/project/kgv/cargofmt)
[![Crates](https://img.shields.io/crates/v/cargofmt.svg)](https://crates.io/crates/cargofmt)
[![Docs](https://docs.rs/cargofmt/badge.svg)](https://docs.rs/cargofmt)
[![License](https://img.shields.io/crates/l/cargofmt)](#license)

A tool for formatting manifest according to style guidelines.

See [style guideline](./STYLE-GUIDELINE.md) for more.

**Work in progress!**

Note the `0.*` version: it means that the code is very much experimental. You
shouldn’t use this crate for any serious project yet.

## Install

`cargo install --git https://github.com/kgv/cargofmt`

## Usage

```text
USAGE:
    cargofmt [FLAGS] [OPTIONS] [MANIFEST_FILES]...
    cargofmt <SUBCOMMAND>

ARGS:
    <MANIFEST_FILES>...    Sets the manifest files to format [default: Cargo.toml]

FLAGS:
    -b, --backup     Backup any modified files
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --config-path <CONFIG_PATH>    Recursively searches the path for the config file [default: ./]
        --output <OUTPUT>              Output type [default: stdout]  [possible values: file, stdout]

SUBCOMMANDS:
    config    Manipulate config
    help      Prints this message or the help of the given subcommand(s)
```

### `cargofmt config`

<details>

```text
USAGE:
    cargofmt config [OPTIONS] [TYPE]

ARGS:
    <TYPE>    Config type [default: active]  [possible values: active, default, diff]

FLAGS:
    -h, --help    Prints help information

OPTIONS:
        --format <FORMAT>    Format type [default: toml]  [possible values: json, ron, toml]
        --output <OUTPUT>    Output type [default: stdout]
```

</details>

## Settings

Order:

- "Unordered" - as is,
- "Alphabetic" - alphabetic order,
- ["name", "version", "authors"] - enumeration order or else alphabetic order.

Inline:

- "Auto" - depends on line length.
- "None" - never inline,
- `0..` - inline starting at level (0 - always inline).

<details><summary>Examples</summary>

for key `a`

`inline = 0`:

```toml
a = { b = { c = { d = "d", e = "e" } } }
```

`inline = 1`:

```toml
[a]
b = { c = { d = "d", e = "e" } }
```

`inline = 2`:

```toml
[a.b]
c = { d = "d", e = "e" }
```

`inline = 3` or more or `inline = "None"`:

```toml
[a.b.c]
d = "d"
e = "e"
```

</details>

## Todo

- [x] comments,
- [ ] diff,
- [ ] more cli options.

## Dedication

To my grannies: **Ann** and **Rimma** and grandpas: **Alexander** and
**George**.
