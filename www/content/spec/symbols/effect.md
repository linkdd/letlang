---
title: Effect
layout: spec
weight: 3
---

# Effect

An effect represents the signature of a side effect.

An expression can delegate to the caller the handling of a side effect, doing so
will interrupt the function in order to call the handler, and resume the
execution with the value returned by the handler.

The effect's signature MUST be used for type-checking at the handler's
boundaries, meaning:

 - arguments are type-checked before delegating the side effect to the handler
 - the value returned by the handler is type-checked before resuming the
   function's execution

### Syntax

{% apply grammkit %}
effect_definition
  = "effect" "[" signature "]"

signature
  = "(" (type_expression ("," type_expression)* ","?)? ")" "->" type_expression
{% endapply %}


### Example

```letlang
let log: effect[(string) -> @ok];

let read_line: effect[() -> string];

let sort_compare<t>: effect[(t, t) -> bool];
```
