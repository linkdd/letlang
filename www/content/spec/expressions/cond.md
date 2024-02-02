---
title: Conditional block
layout: spec
weight: 8
---

# Conditional block

A `cond` expression allows branching based on conditions.

### Syntax

{% apply grammkit %}
cond_expression = "cond" "{" (cond_branch ",")+ cond_else_branch ","? "}"

cond_branch = expression "->" branch_body

cond_else_branch = "else" "->" branch_body

branch_body
  = expression
  / "{" (expression ";")+ "}"
{% endapply %}


### Example

```letlang
cond {
  age < 18 -> "minor",
  age < 60 -> "adult",
  else -> "senior",
};
```

### Semantics

Every conditional expression MUST return a boolean (`true` or `false`). If they
don't, an exception MUST be thrown, the value of that exception is defined by
the implementation.

If a conditional expression throws an exception, that exception MUST NOT be
ignored, and MUST bubble up.

The body of the branch with the first conditional expression that returns `true`
MUST be evaluated. The `cond` expression MUST evaluate to the value this branch
evaluates to.

The conditional expressions following one that returned `true` MUST NOT be
evaluated (this is short-circuiting).

If no conditional expression returns `true`, the default `else` branch MUST be
evaluated.

Each branch MUST create a new scope.
