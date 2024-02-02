---
title: Function
layout: spec
weight: 2
---

# Function

A function is a callable symbol.

Type checking MUST happen at the function boundaries, meaning:

 - arguments are type-checked before the function is executed
 - the return value is type-checked after the function is executed

Functions can be marked as **tail-recursive**. Such functions MUST be unrolled
as a loop to avoid stack overflows.

Tail-recursive functions MUST return either:

 - `recurse[...]`: to indicate that we need to iterate again with new arguments
 - `final[...]`: to indicate that we should exit the loop and return a value

### Syntax

{% apply grammkit %}
function_definition
  = ("func" / "tailrec")
  "[" signature "]"
  "{" pattern_matching_clause ("," pattern_matching_clause)* ","? "}"

signature
  = "(" (type_expression ("," type_expression)* ","?)? ")" "->" type_expression
{% endapply %}


### Example

```letlang
let natural: class[int] {
  (n) -> n >= 0,
};

let factorial_impl: tailrec[(natural, natural) -> natural] {
  (0, acc) -> final[acc],
  (n, acc) -> recurse[n - 1, n * acc],
};

let factorial: func[(natural) -> natural] {
  (0) -> 1,
  (1) -> 1,
  (n) -> factorial_impl(n, 1),
};
```

### Semantics

The return value of a function MUST be the value the matching clause evalutes
to.

For tail-recursive functions, the return value MUST be the value passed with
`final[...]`.
