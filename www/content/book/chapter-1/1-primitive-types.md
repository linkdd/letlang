---
title: 1.1. Primitive Types
description: Description of Letlang's builtin types
prev: /book/chapter-1
next: /book/chapter-1/2-containers
---

# Booleans

The class is named `boolean` and contains the values `true` and `false`:

```letlang
true is boolean;   # true
false is boolean;  # true
```

They are implemented using the Rust type `bool`.

# Numbers

The classes are named `int` and `number`. The class `number` contains all the
numerical values, while the class `int` only contains the ones with a fractional
part equal to `0`:

```letlang
42 is number;   # true
42 is int;      # true

2.3 is number;  # true
2.3 is int;     # false
```

They are implemented using the Rust type `f64`.

> **NB:** Integers are also stored as a `f64`, meaning they have a maximum
> precision of **53 bits**.

# Strings

The class `string` contains all the UTF-8 character strings:

```letlang
"hello world" is string;  # true
```

They are implemented using the Rust type `String`.

# Atoms

Atoms are user-defined symbols, starting with the character `@`. The class
`atom` contains all such values:

```letlang
@ok is atom;
@hello is atom;
```

During compilation, the string representation of the atom is interned and
converted to the Rust type `usize`. Within the **Letlang** runtime, they are
stored using the following structure:

```rust
struct Atom(usize);
```

# Value types

Every letlang value can be used as a type which contains only that specific
value:

```letlang
42 is 42;       # true
42 is "hello";  # false
```

# Implementation detail

Every **Letlang** value is represented by the following Rust enum:

```rust
struct Atom(usize);

enum Value {
  Boolean(bool),
  Number(f64),
  String(String),
  Atom(Atom),
  // container types, see next section
  // function types, see next chapter
}
```
