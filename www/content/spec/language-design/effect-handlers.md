---
title: Effect Handlers
weight: 6
---

# Declaring effect signatures

Every effect **MUST** have a signature used for type-checking.

**Syntax:**

```bnf
<effect-decl-statement> :=
  [ "pub" ] "effect" <identifier> "(" <effect-call-parameters> ")" "->" <type-ref> ";"
  ;

<effect-call-parameters> :=
  <effect-call-parameter> ("," <effect-call-parameter>)*
  ;

<effect-call-parameter> :=
  <identifier> ":" <type-ref>
  ;
```

**Example:**

```letlang
class loglevel(level: @debug | @info | @warning | @error);

effect log(level: loglevel, message: string) -> @ok;
```

# Performing effects or throwing exceptions

**Syntax:**

```bnf
<expression> :=
  | ...
  | <expression-term>
  | <expression-unop>
  | ...
  ;

<expression-term> :=
  | ...
  | <effect-call>
  | ...
  ;


<expression-unop> :=
  <unary-operator> <expression>
  ;

<unary-operator> :=
  | ...
  | "throw"
  | ...
  ;

<effect-call> :=
  "perform" <identifier> ("::" <identifier>)*
  "(" <expression> ("," <expression>)* ")"
  ;
```

**Example:**

```letlang
perform log(@info, "This is an effect");
throw (@error, "This is an exception");
```

# Intercepting effects or catching exceptions

**Syntax:**

```bnf
<expression-term> :=
  | ...
  | <do-block>
  | ...
  ;

<do-block> :=
  "do" "{" <proposition>+ "}"
  <do-block-intercept>*
  <do-block-catch>*
  ;

<do-block-intercept> :=
  "intercept" <identifier> "(" <pattern> ("," <pattern>)* ")"
  "{" <proposition>+ "}"
  ;

<do-block-catch> :=
  "catch" <pattern> "{" <proposition>+ "}"
  ;
```

**Example:**

```letlang
do {
  # block body
}
intercept log(@info, message) {
  # handler body
}
intercept log(level, message) {
  # handler body
}
catch (@error, reason) {
  # handler body
};
```

A `do{}` block **MAY** define effect and/or exception handlers to intercept
and/or catch effects and/or exceptions with the block's body.

When an effect is intercepted, the execution **MUST** resume at the location
where the effect was triggered, and it **MUST** evaluate to the value returned
by the effect handler.

When an exception is caught, the `do{}` block **MUST** evaluate to the value
returned by the exception handler.

When no exception is thrown within a `do{}` block, it **MUST** evaluate to the
value returned by the block's body.

If an effect and/or exception is triggered with the `do{}` block's body and not
intercepted and/or catched by the block's handlers, it **MUST** be propagated.

If an effect and/or exception is never intercepted and/or caught, it **MUST** be
propagated to the runtime.

The runtime **MUST** provide handlers for the following builtin effects:

```letlang
module std::io;

effect __println(message: string) -> @ok;
effect __readline(prompt: string) -> (@ok, string) | (@error, @eof) | (@error, (@io, string));
```

```letlang
module std::proc;

effect __send(proc: pid, message: any) -> @ok | (@error, @dead_process);
effect __link(proc: pid) -> @ok | (@error, @dead_process);
effect __unlink(proc: pid) -> @ok | (@error, @dead_process);
```

The runtime **MAY** provide handlers for implementation-defined effects.

If an unknown effect is propagated to the runtime, it **MUST** crash the Letlang
process.

If an exception is propagated to the runtime, it **MUST** crash the Letlang
process.

Effects and/or exceptions triggered in an effect/exception handler **MUST NOT**
be intercepted and/or caught by the current `do{}` block and **MUST** be
propagated:

```letlang
do {
  do {
    throw (@error, @test);
  }
  catch (@error, reason) {
    throw (@error2, reason)
  }
  catch (@error2, reason) {
    # never executed
    @ok
  };
}
catch (@error2, reason) {
  # will catch the exception from the nested block
};
```
