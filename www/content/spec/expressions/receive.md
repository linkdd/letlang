---
title: Receive block
layout: spec
weight: 9
---

# Receive block

A `receive` expression is used to wait for a signal on the current process.

A signal MUST be a tuple containing 3 elements:

 - the type of signal, as an atom (`@message` or `@exited`)
 - the PID of the process from which the signal originated
 - the payload of the signal

### Syntax

{% apply grammkit %}
receive_expression
  = "receive" "{" pattern_matching_clause ("," pattern_matching_clause)* ","? "}"
  receive_after_expression?

receive_after_expression = "after" expression "{" (expression ";")+ "}"
{% endapply %}


### Example

```letlang
receive {
  (@message, proc_id, msg) -> {
    # ...
  },
  (@exited, proc_id, reason) -> {
    # ...
  },
}
after 1000 {
  # ...
};
```

### Semantics

A `receive` expression MUST block until a signal is received.

The `receive` expression MUST evaluate to the value the matching clause
evaluates to.

If no clause matched, an exception MUST be thrown. The value of that exception
is defined by the implementation.

If present, the `after` part of the `receive` expression MUST add a timeout,
after which we no longer wait for a signal.

The expression of the `after` part MUST evaluate to an integer, which is the
timeout in milliseconds. If it does not evaluate to an integer, an exception
MUST be thrown. The value of that exception is defined by the implementation.

After the timeout, the body of the `after` part MUST be evaluated. In such case,
the `receive` expression MUST evaluate to the value the body evaluates to.

The body of the `after` part MUST create a new scope.
