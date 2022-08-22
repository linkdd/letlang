---
title: 4.1. Functions
description: Overview of Letlang's function declarations
prev: /book/chapter-4/
next: /book/chapter-4/2-processes
---

# Introduction

A function is a sequence of propositions (`let` expressions, or expressions)
always returning a value.

It may take type parameters, or call parameters. The value returned by the last
proposition is the return value of the function.

> **NB:** The only form of early return possible is by throwing an exception.

Type checking happens at the function boundaries:

 - when a function is called, its call parameters are type-checked
 - when the function returns, its return value is type-checked

**Syntax:**

```bnf
<function-statement> :=
    [ "pub" ] "func" <identifier>
    [ <function-type-params> ]
    "(" [ <function-call-params> ] ")"
    "->" <type-ref> "{"
    <proposition>+
    "}"
    ;

<function-type-params> :=
    "<" <identifier> ("," <identifier>)* ">"
    ;

<function-call-params> :=
    <function-call-param> ("," <function-call-param>)*
    ;

<function-call-param> :=
    <identifier> ":" <type-ref>
    ;
```

# Examples

Simple pure function:

```letlang
func adder(a: number, b: number) -> number {
  a + b;
}
```

Simple pure generic function:

```letlang
func swap<T, U>(a: T, b: U) -> (U, T) {
  (b, a);
}
```

Function with side effects:

```letlang
effect log(msg: string) -> @ok;

func task() -> @ok {
  @ok := perform log("hello");
  @ok := perform log("world");
}
```

Early return with exceptions:

```letlang
import std::math;

func task(a: number) -> @ok {
  cond {
    a >= 0 => {
      std::math::sqrt(a);
    },
    else => {
      throw (@error, "no complex numbers here");
    },
  };
}
```

# Implementation details

A function is a structure implementing the builtin trait `Function`.

The structure's members holds the type parameters. Calling the function results
in instantiating this structure and calling the `call()` function from the
`Function` trait:

```letlang
func foo() -> @ok {
  # do stuff
  @ok;
}
```

Is translated to:

```rust
mod symbol_foo {
  struct func_foo;

  impl Function for func_foo {
    fn call(
      &self,
      context: &mut TaskContext,
      args: Vec<Value>,
    ) -> FunctionContinuation {
      // 1. type check arguments

      // 2. execute function's body
    }
  }
}
```

The `call()` function itself is a generator, implemented with the Rust crate
[genawaiter](https://docs.rs/genawaiter/) (because generators are not
[stable yet](https://doc.rust-lang.org/nightly/unstable-book/language-features/generators.html)).

When calling the function, the runtime will execute the generator until
completion, side effects and exceptions interrupts the function execution:

```rust
let task_args: Vec<Value> = vec![];
let mut block = func.call(&mut context, task_args);

let ignored = Value::Boolean(bool::default());
let mut state = block.resume_with(ignored);

loop {
  match &state {
    GeneratorState::Yielded(FunctionInterruption::Effect { name, args }) => {
      // try to handle side effect

      if unknown_side_effect {
        // print debug information
        return Err(RuntimeError::EffectNotImplemented);
      }
      else {
        let effect_return_value = { /* ... */ };
        state = block.resume_with(effect_return_value);
      }
    },
    GeneratorState::Yielded(FunctionInterruption::Exception(exc)) => {
      // print exception
      return Err(RuntimeError::UncaughtException);
    },
    GeneratorState::Complete(val) => {
      // type check function's return value
      break;
    }
  }
}
```