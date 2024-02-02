---
title: Destructuring
layout: spec
weight: 2
---

# Destructuring

### Syntax

{% apply grammkit %}
destructuring_expression = pattern ":=" expression
{% endapply %}


### Example

```letlang
(@ok, val) := (@ok, 42);
```

### Semantics

A destructuring expression MUST evaluate to the matched value, allowing chaining:

```letlang
# evaluates to (1, 2)
(a, 2) := (1, b) := (1, 2);
# a = 1, b = 2
```

If the pattern on the left does not match the expression on the right, an
exception MUST be thrown. The value of this exception is defined by the
implementation:

```letlang
@ok := @error;
# equivalent to
throw @implementation_defined_error;
```
