---
title: 3.2. Loops
description: Overview of Letlang's looping mechanism
prev: /book/chapter-3/1-conditional-branching
next: /book/chapter-3/3-side-effects
---

# Introduction

Letlang provides only one loop mechanism, the `tailrec` function.

A tail recursive function returns either:

 - `final[value]` to exit the loop
 - `recurse[args...]` to keep iterating with new arguments

# Example

```letlang
tailrec len<T>(items: list<T>, acc: int) -> int {
  match items {
    [] => final[acc],
    [_ | tail] => recurse[tail, acc + 1],
  };
}
```
