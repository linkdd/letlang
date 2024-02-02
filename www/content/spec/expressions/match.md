---
title: Match block
layout: spec
weight: 7
---

# Match block

A `match` expression allows to branch by pattern on an expression.

### Syntax

{% apply grammkit %}
match_expression
  = "match" expression
  "{" pattern_matching_clause ("," pattern_matching_clause)* ","? "}"
{% endapply %}


### Example

```letlang
match result {
  (@ok) -> {
    # ...
  },
  (@error) -> {
    # ...
  },
};
```

### Semantics

A `match` expression MUST evaluate to the value the matching clause evaluates to.

If no clause matches, and exception MUST be thrown, the value of that exception
is defined by the implementation:

```letlang
match 42 {
  (41) -> @ok,
};

# is equivalent to:

match 42 {
  (41) -> @ok,
  (_) -> throw @implementation_defined_error,
};
```
