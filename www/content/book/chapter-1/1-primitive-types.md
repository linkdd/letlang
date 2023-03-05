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

Every Letlang number or integer is a 64-bits float (see
[IEEE-754](https://standards.ieee.org/ieee/754/6210/) specification).

# Strings

The class `string` contains all the UTF-8 character strings:

```letlang
"hello world" is string;  # true
```

# Atoms

Atoms are user-defined symbols, starting with the character `@`. The class
`atom` contains all such values:

```letlang
@ok is atom;
@hello is atom;
```

# Value types

Every letlang value can be used as a type which contains only that specific
value:

```letlang
42 is 42;       # true
42 is "hello";  # false
```
