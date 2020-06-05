# Cargofmt

A tool for formatting manifest according to style guidelines.

See [style guideline](./STYLE-GUIDELINE.md) for more.

**Work in progress!**

Note the `0.*` version: it means that the code is very much experimental. You
shouldn’t use this crate for any serious project yet.

## Install

`cargo install --git https://github.com/kgv/cargofmt`

## Usage

Looks for the `Cargo.toml` file starting from the current directory, formats and
prints it to stdout:

`cargofmt`

Looks for the `Cargo.toml` file starting from the current directory, formats and
overwrites it:

`cargofmt --output=file`

Looks for the `Cargo.toml` file starting from the current directory, formats and
overwrites it with backup:

`cargofmt --backup --output=file`

Looks for the `Cargo.toml` file starting from the `./config` directory, formats
and overwrites it:

`cargofmt --config-path=./config --output=file`

Prints active config to stdout:

`cargofmt config`

Prints default config to stdout:

`cargofmt config default`

Writes active config to file:

`cargofmt config --output=config.toml`

## Settings

Order:

- "Unordered" - as is,
- "Alphabetic" - alphabetic order,
- ["name", "version", "authors"] - enumeration order or else alphabetic order.

Inline:

- "Auto" - depends on line length.
- "None" - never inline,
- `0..` - inline starting at level (0 - always inline).

### Examples

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

## Todo

- [x] comments,
- [ ] diff,
- [ ] more cli options.

## Dedication

To my grannies: **Ann** and **Rimma** and grandpas: **Alexander** and
**George**.
