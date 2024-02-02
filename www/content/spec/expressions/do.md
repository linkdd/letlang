---
title: Do block
layout: spec
weight: 10
---

# Do block

A `do` expression creates a capture scope to intercept effects and/or catch
exceptions.

### Syntax

{% apply grammkit %}
do_expression = "do" "{" (expression ";")+ "}" intercept_block* catch_block?

intercept_block
  = "intercept" effect_ref
  "{" pattern_matching_clause ("," pattern_matching_clause)* ","? "}"

effect_ref
  = symbol_path
  ("<" type_expression ("," type_expression)* ","? ">")?

catch_block
  = "catch"
  "{" pattern_matching_clause ("," pattern_matching_clause)* ","? "}"
{% endapply %}


### Example

Consider the following effects:

```letlang
let log: effect[(string) -> @ok];
let compare<t>: effect[(t, t) -> @lesser | @greater | @equal];
```

```letlang
do {
  # ...
}
intercept log {
  (msg) -> @ok,
}
intercept compare<int> {
  (a, b) when a < b -> @lesser,
  (a, b) when a > b -> @greater,
  (a, b) when a = b -> @equal,
}
catch {
  ((@error, reason)) -> {
    # ...
  },
};
```

### Semantics

The body of a `do` expression MUST create a new scope.

The `do` expression MUST evaluate to the value its body evaluates to.

When an effect is performed, or an exception is thrown, the runtime MUST check
if one of the `intercept` or `catch` block matches the effect or exception.

When an effect is intercepted, if no clause matches, the effect MUST bubble up.

When an exception is caught, if no clause matches, the exception MUST bubble up.

When an exception is caught and a clause matched, the `do` expression MUST
evaluate to the value the clause evaluates to.
