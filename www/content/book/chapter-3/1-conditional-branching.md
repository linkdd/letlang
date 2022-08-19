---
title: 3.1. Conditional Branching
description: Overview of Letlang's conditional expression
prev: /book/chapter-3/
next: /book/chapter-3/2-loops
---

# Introduction

In **Letlang**, conditional branching is achieved with `cond` expressions.

A `cond` expression will evaluate the first branch whose condition is true. If
none of the conditions are true, the `cond` expression will evaluate the default
branch (`else`).

**Syntax:**

```bnf
<cond-expression> :=
    "cond" "{"
    (<cond-expression-branch> ",")+
    <cond-expression-default-branch>
    "}"
    ;

<cond-expression-branch> :=
    | <expression> "=>" "{" <proposition>+ "}"
    | <expression> "=>" <expression>
    ;

<cond-expression-default-branch> :=
    | "else" "=>" "{" <proposition>+ "}"
    | "else" => <expression>
    ;
```

> **NB:** The `cond` default branch is mandatory in order to always return a
> value.

Branch conditions are evaluated sequentially in the order they are written in
the code. Once a condition evaluates to `true`, following conditions are ignored
and not evaluated.

# Example

```letlang
a := 30;

category := cond {
  a < 18 => "minor",
  a < 60 => "adult",
  else => "senior",
};
```
