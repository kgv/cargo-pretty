# Cargofmt

A tool for formatting manifest according to style guidelines.

Note the `0.0.1` version: it means that the code is very much experimental. You
shouldn’t use this crate for any serious project yet.

See [style guideline](./STYLE_GUIDELINE.md) for more.

## Settings

Order:

- "Forward" - alphabetical order,
- "Backward" - reverse alphabetical order,
- ["name", "version", "authors"] - enumeration order or else as is.

Inline:

- "Auto" - depends on line length.
- "Never" - never inline,
- "Always" - always inline,
- `1..` - level inline.

`inline = "Never"`:

```toml
[a.b.c]
d = "d"
e = "e"
```

`inline = 1`:

```toml
[a.b]
c = { d = "d", e = "e" }
```

`inline = 2` or more or `inline = "Always"`:

```toml
[a]
b = { c = { d = "d", e = "e" } }
```

## Dedication

To my grannies: **Ann** and **Rimma** and grandpas: **Alexander** and
**George**.
