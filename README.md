# Cargofmt

A tool for formatting manifest according to style guidelines.

## Settings

Order:

- "Forward" - alphabetical order,
- "Backward" - reverse alphabetical order,
- ["name", "version", "authors"] - enumeration order or else as is.

Inline:

- "Auto" - depends on line length.
- "Never" - never inline,
- "Always" - always inline,
- `1..` - level inline,

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
