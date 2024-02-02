---
title: Spawning processes
layout: spec
weight: 14
---

# Spawning processes

A `spawn` expression is used to execute a function in a new concurrent
"process".

### Syntax

{% apply grammkit %}
spawn_expression = "spawn" function_call_expression
{% endapply %}


### Example

```letlang
(@ok, pid) := spawn foo("bar");
```

### Semantics

A `spawn` expression MUST return a value of type `(@ok, pid)` or `(@error, term)`.
