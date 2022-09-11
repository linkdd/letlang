---
title: 2.1. Match operator
description: Overview of Letlang's variable assignment
prev: /book/chapter-2/
next: /book/chapter-2/2-inline-typing
---

# Introduction

The match operator (`:=`) is the simplest for of pattern matching in
**Letlang**.

It tries to match a left-hand side pattern to a right-hand side expression. If
successful, the whole expression returns the value on the right-hand side. If
the matching fails, a run-time error occurs.

# Examples:

The most simple example is to bind a single value to a single variable:

```letlang
a := 1;  # a is now bound to the value 1
a := 2;  # a is now bound to the value 2
```

But we can also bind multiple variables at once using **tuples**:

```letlang
(a, b) := (1, 2);  # a is bound to 1, and b is bound to 2
```

Or **structures**:

```letlang
{x: a, y: b} := {x: 1, y: 1};  # a is bound to 1, and b is bound to 2
```

Or **lists**:

```letlang
[a, b] := [1, 2];  # a is bound to 1, and b is bound to 2
```

We can also extract the head and tail of a list using pattern matching:

```letlang
[head | tail] := [1, 2, 3];  # head is bound to 1, and tail is bound to [2, 3]
```

Literal values can be used as a pattern:

```letlang
(@ok, val) := (@ok, 42);   # val is bound to 42
```

Because the match operator returns the right-hand side value, it can be chained:

```letlang
(a, 2) := (1, b) := (1, 2);  # a is bound to 1, and b is bound to 2
```

We can also match against an expression using `$()`:

```letlang
a := 1;
(@ok, $(a + 1)) := (@ok, 2);
```
