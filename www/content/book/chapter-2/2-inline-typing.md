---
title: 2.2. Inline typing
description: Enforcing constraints for unbound variables
prev: /book/chapter-2/1-match-operator
next: /book/chapter-2/3-match-branching-expression
---

# Introduction

It is possible to declare constraints to unbound variables in order to restrict
the values it can match.

This is done by using a `let` proposition.

# Examples

Enforcing the type for an unbound variable:

```letlang
let a: number;
let b: string;

a := 1;
b := 2;  # error, 2 is not a string
```

Restricting the value of an unbound variable:

```letlang
let a: number;
let b: number {
  b > a
};

a := 1;
b := 0;  # error, `0 > 1` is false
```

# Assertions

If the variable is already bound, the `let` proposition acts as an assertion:

```letlang
a := 1;

let a: string;  # error, 1 is not a string
```

```letlang
a := 1;

let a: number {
  a < 0
};  # error, `1 < 0` is false
```

If successful, the `let` proposition returns the atom `@ok`:

```letlang
n := 1;

@ok := do {
  let n: number { n > 0 };
};
```

> **NB:** A `let` proposition for an unbound variable is always successful.
