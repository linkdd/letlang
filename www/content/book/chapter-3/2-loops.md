---
title: 3.2. Loops
description: Overview of Letlang's looping mechanism
prev: /book/chapter-3/1-conditional-branching
next: /book/chapter-3/3-side-effects
---

# Introduction

Letlang provides only one loop mechanism, the `loop` expression.

A `loop` expression will loop indefinitely until a `break` expression is met.

`loop` expressions are labelled, and the `break` expression provides the label
of the loop it is breaking out of, as well as the value to return.

**Syntax:**

```bnf
<loop-expression> :=
    "loop" "(" <identifier> ")" "{"
    <proposition+>
    "}"
    ;

<break-expression> :=
    "break" "(" <identifier> ")" <expression>
    ;
```

# Examples

Simple loop with only one iteration:

```letlang
42 := loop(simple) {
  # do stuff

  break(simple) 42;
};
```

Simple loop iterating until a condition is met:

```letlang
42 := loop(simple) {
  # do stuff

  @ok := cond {
    check_condition() => {
      break(simple) 42;
    },
    else => {
      @ok;
    }
  };
};
```

Breaking out of nested loop:

```letlang
42 := loop(outer) {
  loop(inner) {
    break(outer) 42;
  };
};
```
