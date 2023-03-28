---
title: Type System
weight: 2
---

# Core Concept

In Letlang, values are not assigned a single type. Instead, each type determines
which values it contains.

For example:

 - `42` is an `int`, a `number`, and the singleton type `42`
 - `"hello"` is a `string` and the singleton type `"hello"`

We can consider a type definition to be some kind of function which returns
`true` or `false` wether the value belongs to that type.

# Primitive types

The Letlang runtime **MUST** provide the following types:

 - `bool` which contains the values `true` and `false`
 - `int` which contains every 64-bits floating-point number with the fractional part equal to 0
 - `number` which contains every 64-bits floating-point number
 - `string` which contains every UTF-8 encoded strings
 - `atom` which contains every atom

> **NB:** An atom is a developer-defined symbol, for example: `@ok`, `@error`, `@hello_world`.

Additionally, the Letlang runtime **MUST** provide the following types:

 - `pid` representing Letlang process identifiers
 - `file` representing file descriptors
 - `socket` representing socket descriptors

Those types **MUST NOT** be forgeable by the developer, only the Letlang runtime
should be able to create values of those types.

Literal values **MUST** match the following syntax:

{{< grammar-block "literal" >}}

# Container types

The Letlang runtime **MUST** provide the following container types:

 - `list<T>`, a generic type representing a list whose elements are of type `T`,
   for example: `[1, 2, 3]` is a `list<int>`
 - tuples, for example: `(int, string)` contains values such as `(42, "hello")`
 - structures, for example: `{foo: string}` contains values such as `{foo: "bar"}`

The Letlang runtime **MUST NOT** provide a `tuple` or `struct` type containing
"every tuples" or "every structures".

# Singleton types

Every value **MUST** be its own singleton type, for example:

 - `42` is a singleton type containing only the value `42`
 - `"foo"` is a singleton type containing only the value `"foo"`
 - `(@ok, 42)` is a singleton type containing only the value `(@ok, 42)`

# Combining types

The Letlang runtime **MUST** allow types to be combined using `|` (or), `&`
(and) and/or `!` (not).

For example:

 - `int | string` is a type containing all integers and all strings
 - `!int` is a type containing all values, except integers
 - `number & !int` is a type containing all numbers except integers

This can be used to define tagged enumerations, such as:

 - `(@ok, int) | (@error, string)`, similar to Rust's Result type
 - `@none | (@some, int)`, similar to Rust's Option type

# Functional types

Every function is a value whose type is its signature, for example:

 - `func foo(bar: string) -> @ok { ... }` belongs to the type `func[(string) -> @ok]`
 - `func add(a: int, b: int) -> int { ... }` belongs to the type `func[(int, int) -> int]`

# Referencing types

A type reference is defined using the following syntax:

{{< grammar-block "typeref" >}}

# Defining types

A custom type is defined using a `class`:

 - it may have a public visibility (modules importing this module may reference it)
 - it must have a constructor parameter (defining the structure of the values it contains)
 - it may have a predicate to further restrain what values it contains

The type's predicate (if present) **MUST** return a `bool`.

The `class` is defined using the following syntax:

{{< grammar-block "class" >}}

> **NB:** `proposition_let` and `proposition_expr` rules will be defined later.

The value of the last proposition of the class's predicate evaluates to is the
return value of the predicate.

Example:

```letlang
module example;

class even(n: int) {
  n % 2 = 0;
}

class odd(n: int & !even);
```

# Type Checking

The Letlang runtime **MUST** perform type checking at the function's boundaries:

 - type checking the function arguments before its invocation
 - type checking the function's return value after its invocation

The Letlang runtime **MUST** perform type checking when binding a value to a
variable.

The Letlang runtime **MUST** perform type checking when requested manually by
the developer.
