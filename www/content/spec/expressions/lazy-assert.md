---
title: Lazy Assertions
layout: spec
weight: 6
---

# Lazy Assertions

Lazy assertions rely on the `let` expression in order to constrain the value of
future variable bindings.

They provide 2 kinds of constraints:

 - **type constraints:** only values belonging to this type can be bound to this
   name
 - **guard expression:** when binding a value to a name, the guard expression
   MUST evaluate to true

### Syntax

{% apply grammkit %}
let_expression
  = "let" "(" let_type_bind ("," let_type_bind)* ","? ")"
  let_guard? "{" (expression ";")+ "}"

let_type_bind = identifier ":" type_expression

let_guard = "with" expression
{% endapply %}


### Example

```letlang
let (x: int, y: int) with x > y {
  x := 1;
  y := 2;  # throws an exception because 1 > 2 is false
};
```

### Semantics

`let` expression MUST create a new scope.

When destructuring an expression using the pattern matching operator `:=`, the
runtime MUST verify that all `let` constraints that are in scope are satisfied.

If a `let` constraint is not satisfied, pattern matching MUST fail.

When nesting `let` expressions with a constraint on the same name, all
constraints MUST be satisfied:

```letlang
let (x: int) {
  let (x: string) {
    # ...
  };
};

# is equivalent to:

let (x: int & string) {
  # ...
};
```

The guard expression of a `let` expression MUST return a boolean (`true` or
`false`).

If an exception is thrown in the guard expression, it SHOULD be ignored and be
equivalent to returning `false`.
