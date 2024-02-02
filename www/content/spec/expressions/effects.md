---
title: Effects
layout: spec
weight: 12
---

# Effects

Letlang provides the ability to delegate a computation to the caller. This is
done by first declaring the signature of an effect (see
[this page](./spec/symbols/effect)), then using the `perform` keyword, the
callee can "call" an effect handler defined by the caller using a `do`
expression.

### Syntax

{% apply grammkit %}
effect_call_expression
  = "perform" effect_ref "(" expression ("," expression)* ","? ")"

effect_ref = symbol_path ("<" type_expression ("," type_expression)* ","? ">")?
{% endapply %}


### Example

Consider this effect:

```letlang
let log: effect[(string) -> @ok];
```

```letlang
@ok := perform log("hello world");
```

Or:

```letlang
let compare<t>: effect[(t, t) -> @lesser | @equal | @greater]
```

```letlang
@lesser := perform compare<int>(1, 2);
```

### Semantics

When calling an effect, the arguments MUST be evaluated in order.

Calling an effect MUST interrupt the function's execution. If a matching effect
handler is found in an outer `do` expression, the handler MUST be executed.

The function MUST resume with the handler's return value.

If the handler throws an exception, that exception MUST be thrown from the
effect call-site:

```letlang
let will_throw: effect[() -> @ok];
```

```letlang
@error_caught := do {
  do {
    perform will_throw();
  }
  catch {
    (@oops) -> @error_caught,
  };
}
intercept will_throw {
  () -> throw @oops,
};
```

**NB:** This can be shortened to:

```letlang
@error_caught := do {
  perform will_throw();
}
intercept will_throw {
  () -> throw @oops,
}
catch {
  (@oops) -> @error_caught,
};
```

If no handler is found, the effect call MUST be handled by the runtime.

The runtime provides effect handlers for builtin effects. Those effects will be
handled according to the [runtime specification](./spec/runtime/effects).
However, if the effect is a user-defined effect, the runtime MUST throw an
exception instead. The value of that exception is defined by the implementation.
