---
title: Function Call
layout: spec
weight: 11
---

# Function Call

### Syntax

{% apply grammkit %}
function_call_expression
  = symbol_path ("<" type_expression ("," type_expression)* ","? ">")?
  "(" expression ("," expression)* ","? ")"
{% endapply %}


### Example

```letlang
foo(42, @ok);
```

### Semantics

Function arguments MUST be evaluated in order:

```letlang
foo(bar(), baz());
```

Here, `bar()` is evaluated first, then `baz()`, finally the function `foo` is
called.

If an exception is thrown during the arguments evaluation, this MUST
short-cirtcuit the evaluation of the remaining arguments:

```letlang
foo(throw @error, @unreachable);
```

Here, `throw @error` is evaluated first, but an exception is thrown, therefore
`@unreachable` is never evaluated.
