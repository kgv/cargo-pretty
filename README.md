# Cargofmt

A tool for formatting manifest according to style guidelines.

See [style guideline](./STYLE-GUIDELINE.md) for more.

**Work in progress!**

Note the `0.*` version: it means that the code is very much experimental. You
shouldn’t use this crate for any serious project yet.

## Settings

Order:

- "Unordered" - as is,
- "Alphabetic" - alphabetic order,
- ["name", "version", "authors"] - enumeration order or else alphabetic order.

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

## Todo

- [x] comments,
- [ ] diff,
- [ ] more cli options.

## Dedication

To my grannies: **Ann** and **Rimma** and grandpas: **Alexander** and
**George**.
