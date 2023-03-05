---
title: 4.1. Functions
description: Overview of Letlang's function declarations
prev: /book/chapter-4/
next: /book/chapter-4/2-processes
---

# Introduction

A function is a sequence of propositions (`let` expressions, or expressions)
always returning a value.

It may take type parameters, or call parameters. The value returned by the last
proposition is the return value of the function.

> **NB:** The only form of early return possible is by throwing an exception.

Type checking happens at the function boundaries:

 - when a function is called, its call parameters are type-checked
 - when the function returns, its return value is type-checked

# Examples

Simple pure function:

```letlang
func adder(a: number, b: number) -> number {
  a + b;
}
```

Simple pure generic function:

```letlang
func swap<T, U>(a: T, b: U) -> (U, T) {
  (b, a);
}
```

Function with side effects:

```letlang
effect log(msg: string) -> @ok;

func task() -> @ok {
  @ok := perform log("hello");
  @ok := perform log("world");
}
```

Early return with exceptions:

```letlang
import std::math;

func task(a: number) -> @ok {
  cond {
    a >= 0 => {
      std::math::sqrt(a);
    },
    else => {
      throw (@error, "no complex numbers here");
    },
  };
}
```
