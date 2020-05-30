# Naming conventions

This is naming conventions for general toml elements.

- Toml is a *path*-[*value*][key-value] map.
- A **path** is a comma-separated chain of *path segments*.
- A **path segment** is a *key segment* (table) or *index* (array).
- A **key** is a comma-separated chain of *key segments*.
- A **key segment** is a [*bare key*][key] or [*quoted key*][key].

## Comments

- Any comment is associated with a path.
- Comments can be of two types: pre and post.
- Any path can associate zero or more pre comments and zero or one post comment.

### Comments for primitive value associated path

```toml
# pre comment for `a` path
a = "a" # post comment for `a` path
```

### Comments for table value associated path

```toml
# pre comment for `a` path
[a] # post comment for `a` path
# pre comment for `a.b` path
b = "b" # post comment for `a.b` path
```

Inline:

```toml
# pre comment for `a` path
a = { b = "b" } # post comment for `a` path
```

### Comments for array value associated path

```toml
# pre comment for `a.[0]` path
[[a]] # post comment for `a.[0]` path
# pre comment for `a.[0].b` path
b = "b" # post comment for `a.[0].b` path

# pre comment for `a.[1]` path
[[a]] # post comment for `a.[1]` path
# pre comment for `a.[1].b` path
b = "value" # post comment for `a.[1].b` path
```

Inline:

Any comment inside inline array brackets is associated with a path of the item
of the array (not the array itself).

```toml
# pre comment for `a` path
a = [...] # post comment for `a` path
```

```toml
# pre comment for `a` path
a = [
    ...
] # post comment for `a` path
```

```toml
# pre comment for `a` path
a = [# pre comment for `a.[0]` path
    # pre comment for `a.[0]` path
    [# pre comment for `a.[0].[0]` path
        "b",
        "c",
    ], # post comment for `a.[0]` path
    ["d", "e"],
] # post comment for `a` path
```

### Not recommended

End of file:

```toml
path = "value"
# comment for ???
```

[key-value]: https://github.com/toml-lang/toml#keyvalue-pair
[key]: https://github.com/toml-lang/toml#user-content-keys
