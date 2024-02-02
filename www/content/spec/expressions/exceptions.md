---
title: Exceptions
layout: spec
weight: 13
---

# Exceptions

Exceptions are effects that do not resume the function's execution.

### Syntax

{% apply grammkit %}
throw_expression = "throw" expression
{% endapply %}


### Example

```letlang
throw (@error, "some reason");
```

### Semantics

Throwing an exception MUST interrupt the function's execution. If a matching
exception handler is found in an outer `do` expression, the handler MUST be
executed.

If no handler is found, the current process MUST terminate. The runtime SHOULD
display the exception with a stack trace.

The `throw` expression MUST NOT evaluate to a value. Control MUST NOT be given
back.