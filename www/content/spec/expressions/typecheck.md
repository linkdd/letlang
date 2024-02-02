---
title: Type checking
layout: spec
weight: 3
---

# Type checking

A type-checking expression MUST evaluate to a boolean (`true` or `false`).

### Syntax

{% apply grammkit %}
typecheck_expression = expression "is" "not"? type_expression
{% endapply %}


### Example

```letlang
42 is int;
42 is not string;
42 is (string | int);
```
