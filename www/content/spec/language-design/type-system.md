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

```bnf
<typeref> :=
  | <typeref-value>
  | <typeref-name>
  | <typeref-struct>
  | <typeref-tuple>
  | <typeref-function>
  | <typeref-oneof>
  | <typeref-allof>
  | <typeref-not>
  ;

<typeref-value> :=
  <literal>
  ;

<typeref-name> :=
  <identifier> ("<" <typeref-name-generic-params> ">")?
  ;
<typeref-name-generic-params> :=
  <identifier> ("," <identifier>)*
  ;

<typeref-struct> :=
  "{" <typeref-struct-members> "}"
  ;
<typeref-struct-members> :=
  <typeref-struct-member> ("," <typeref-struct-member>)*
  ;
<typeref-struct-member> :=
  <identifier> ":" <typeref>
  ;

<typeref-tuple> :=
  "(" <typeref-tuple-members> ")"
  ;
<typeref-tuple-members> :=
  <typeref> ("," <typeref>)*
  ;

<typeref-function> :=
  "func" "[" "(" <typeref-function-params ")" "->" <typeref-function-return> "]"
  ;
<typeref-function-params> :=
  <typeref> ("," <typeref>)*
  ;
<typeref-function-return> :=
  <typeref>
  ;

<typeref-oneof> :=
  <typeref> "|" <typeref>
  ;

<typeref-allof> :=
  <typeref> "&" <typeref>
  ;

<typeref-not> :=
  "!" <typeref>
  ;

<literal> :=
  | <literal-bool>
  | <literal-number>
  | <literal-string>
  | <literal-atom>
  ;

<literal-bool> :=
  | "true"
  | "false"
  ;

<literal-number> :=
  | /0b_*[01][_01]*/
  | /0o_*[0-7][_0-7]*/
  | /[1-9][_1-9]*/
  | /0x_*[0-9a-fA-F][_0-9a-fA-F]*/
  | /((\d+\.?\d*)|(\.\d+))(([eE][+-]?)?\d+)?/
  ;

<literal-string> :=
  /"(?:[^"]|\\")*"/
  ;

<literal-atom> :=
  /@(('(?:[^']|\\')+')|([_a-zA-Z][_a-zA-Z0-9]*))/
  ;

<identifier> :=
  /[_a-zA-Z][_0-9a-zA-Z]*/
  ;
```

# Defining types

A custom type is defined using a `class`:

 - it may have a public visibility (modules importing this module may reference it)
 - it must have a constructor parameter (defining the structure of the values it contains)
 - it may have a predicate to further restrain what values it contains

The type's predicate (if present) **MUST** returns a `bool`.

The `class` is defined using the following syntax:

```bnf
<class-statement> :=
  | <class-statement-no-predicate>
  | <class-statement-with-predicate>
  ;

<class-statement-no-predicate> :=
  [ "pub" ] "class" <identifier>
  [ <class-type-parameters> ]
  "(" <class-constructor-parameter> ")"
  ";"
  ;

<class-statement-with-predicate> :=
  [ "pub" ] "class" <identifier>
  [ <class-type-parameters> ]
  "(" <class-constructor-parameter> ")"
  "{" <proposition>+ "}"
  ;

<class-type-parameters> :=
  "<" <identifier> ("," <identifier>)* ">"
  ;

<class-constructor-parameter> :=
  <identifier> ":" <type-ref>
  ;

<proposition> :=
  | <proposition-let>
  | <proposition-expression>
  ;
```

> **NB:** `<proposition-let>` and `<proposition-expression>` rules will be
> defined later.


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
