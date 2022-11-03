---
title: 3.2. Loops
description: Overview of Letlang's looping mechanism
prev: /book/chapter-3/1-conditional-branching
next: /book/chapter-3/3-side-effects
---

# Introduction

Letlang provides only one loop mechanism, the `tailrec` functions.

A tail recursive function returns either:

 - `final[value]` to exit the loop
 - `recurse[args...]` to keep iterating with new arguments

**Syntax:**


```bnf
<tailrec-statement> :=
    [ "pub" ] "tailrec" <identifier>
    [ <tailrec-type-params> ]
    "(" [ <tailrec-call-params> ] ")"
    "->" <type-ref> "{"
    <proposition>+
    "}"
    ;

<tailrec-type-params> :=
    "<" <identifier> ("," <identifier>)* ">"
    ;

<tailrec-call-params> :=
    <tailrec-call-param> ("," <tailrec-call-param>)*
    ;

<tailrec-call-param> :=
    <identifier> ":" <type-ref>
    ;
```

# Example

```letlang
tailrec len<T>(items: list<T>, acc: int) -> int {
  match items {
    [] => final[acc],
    [_ | tail] => recurse[tail, acc + 1],
  };
}
```
