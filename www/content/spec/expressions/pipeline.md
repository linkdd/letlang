---
title: Pipeline
layout: spec
weight: 4
---

# Pipeline

The expression on the left MUST be injected as first argument to the function
call on the right.

### Syntax

{% apply grammkit %}
pipe_expression = expression "|>" function_call_expression
{% endapply %}


### Example

```letlang
1 |> add(5) |> mul(4);
# equivalent to
mul(add(1, 5), 4);
```
