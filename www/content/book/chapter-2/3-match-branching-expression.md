---
title: 2.3. Match Branching Expression
description: Overview of pattern matching based control flow
prev: /book/chapter-2/2-inline-typing
next: /book/chapter-3/
---

# Introduction

You can use a `match{}` expression to match against a sequence of patterns.

If no `match{}` clause matches, a run-time error will occur.

# Example

```letlang
res := match foo() {
  (@ok, val) => val,
  (@error, _reason) => 0
};
```
