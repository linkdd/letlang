---
title: Homepage
---

{{< center >}}
  ![status: WIP](https://img.shields.io/badge/status-WIP-red)
  ![version: 0.0.0](https://img.shields.io/badge/version-v0.0.0-brightgreen)
  ![license: MIT](https://img.shields.io/badge/license-MIT-blue)
{{< /center >}}

{{< hero class="is-small mb-0 pb-0" >}}
{{< markdown >}}
**Letlang** is a general purpose functional programming language.

Implemented in *[Rust](https://www.rust-lang.org/)*, it compiles to Rust,
allowing you to target any platform supported by [LLVM](https://llvm.org).
{{< /markdown >}}
{{< /hero >}}

{{< message class="is-danger has-text-justified mx-6" header="Work In Progress" >}}
**Letlang** is in a very early stage.

The documentation and code examples you may find on this website are **NOT**
definitive and may be subject to change.
{{< /message >}}

---

{{< center >}}
# **Useful resources**
{{< /center >}}

{{< columns >}}
{{< column class="is-one-quarter" >}}
{{< card title="Handbook" headerClass="has-background-info has-text-white" >}}
{{< markdown >}}
Read more about **Letlang**'s features and design.
A good resource to learn about the language.

---
{{< /markdown >}}

{{< center >}}
[Read](/book/)
{{< /center >}}
{{< /card >}}
{{< /column >}}

{{< column class="is-one-quarter" >}}
{{< card title="Code Examples" headerClass="has-background-warning has-text-black" >}}
{{< markdown >}}
Discover code samples from the standard library or implentations of simple UNIX.
tools.

---
{{< /markdown >}}

{{< center >}}
[Read](/examples/)
{{< /center >}}
{{< /card >}}
{{< /column >}}

{{< column class="is-one-quarter" >}}
{{< card title="Language Specification" headerClass="has-background-danger has-text-white" >}}
{{< markdown >}}
Read the language specification by exploring the different **LEP** (**L**etlang
**E**nhancement **P**roposal) documents.

---
{{< /markdown >}}

{{< center >}}
[Read](/lep/)
{{< /center >}}
{{< /card >}}
{{< /column >}}

{{< column class="is-one-quarter" >}}
{{< card title="Download" headerClass="has-background-success has-text-white" >}}
{{< markdown >}}
Github repository containing the source code and setup instructions to get
started with **Letlang**.

---
{{< /markdown >}}

{{< center >}}
[Get started](https://github.com/linkdd/letlang)
{{< /center >}}
{{< /card >}}
{{< /column >}}
{{< /columns >}}

---

{{< center >}}
# **Key features**
{{< /center >}}

## Syntax
{.has-title-underline}

{{< columns >}}

{{< column class="is-half" >}}
{{< markdown >}}
**Letlang** is inspired by mathematics and the following languages:

 - *[Elixir](https://elixir-lang.org/)*
 - *[Python](https://www.python.org/)*
 - *[Rust](https://www.rust-lang.org/)*
 - *[Go](https://go.dev/)*
 - *[TypeScript](https://www.typescriptlang.org/)*
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
module example::main;

import std::io;

pub func main(args: list<string>) -> @ok {
  std::io::println("Hello World");
  @ok;
}
```
{{< /markdown >}}
{{< /column >}}

{{< /columns >}}

## Dynamic Type System
{.has-title-underline}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
In mathematics, a value does not have a single type. Instead, it belongs to one
or more sets:

 - `42` is an integer, a real number, a scalar, ...
 - `(2, 1)` is a 2D vector, a 2x1 matrix, ...

This concept is at the core of **Letlang**'s type system. New types can be
defined using the `class` statement.

A class has a structure, every value matching this structure belongs to the
class.

Optionnaly, a class can define a predicate. Every value belonging to the class
**must** validate the predicate.
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
module example::types;

class even(n: int) {
  n % 2 = 0;
}

class odd(n: int & !even);

class vector(v: {x: number, y: number });

class unit_vector(v: vector) {
  v.x ** 2 + v.y ** 2 = 1
}
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

Using the above definitions, we can assert:

```letlang
42 is even;    # true
42 is odd;     # false
43 is odd;     # true
43 is even;    # false
```

Furthermore, we can compose types together with the operators `|`, `&` and `!`:

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
```letlang
let var: number | string;
```
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
- `var` is either a number or a string
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
```letlang
let var: int & !even;
```
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
- `var` is an integer and must not be even
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

## Generics
{.has-title-underline}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
Classes and functions can require type parameters.

Those parameters can then be used within the definition of the class or
function.

Multiple parameters can be specified.
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
class vector<T>(xy: (T, T));

func swap<A, B>(a: A, b: B) -> (B, A) {
  (b, a);
}
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
These parameters need to be specified to use the generic class or function.

But most of the time, the compiler can infer the types.
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
let v: vector<number>;

x, s := swap("hello", 42);
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
Thanks to this feature, a Result type (similar to Rust's) can be defined as
follows:
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
class ok<T>(v: (@ok, T));
class err<T>(v: (@error, T));
class result<T, E>(v: ok<T> | err<E>);
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

Leading to the following equalities:

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
```letlang
(@ok, 42) is result<number, string>;
```
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
- this is `true`
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
```letlang
(@ok, 42) is ok<number>;
```
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
- this is `true`
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
```letlang
(@error, "wrong") is result<number, string>;
```
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
- this is `true`
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
```letlang
(@error, "wrong") is ok<number>;
```
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
- this is `false`
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

## Pattern matching
{.has-title-underline}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
The `:=` operator is **not** an assignment operator, it is the
**pattern matching** operator.

If the pattern on the left does not match the value on the right, an exception
is thrown.

Undefined variables used in a pattern will be bound to the value they match.
They cannot be bound again to a new value.

The operator returns the right-value as a result, allowing the operator to be
chained.

Pattern matching is especially useful to extract information out of complex
data structures.
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
(a, b) := (1, 2);
a := 2; # a is 1, 1 does not match 2 -> error

(@ok, val) := (@ok, 42);

@ok := (@error, "oops"); # error

(x, 2) := (1, y) := (1, 2);
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

## Inline typing
{.has-title-underline}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
Before being bound to a value, it is possible to declare constraints on a
variable using the `let` expression.

When binding a value to a constrained variable, it must validate all the defined
predicates. If it fails validate, an exception will be thrown.
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
let a: number;
let b: number { b > 0, b > a };

a := 5;
b := 4; # error
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

## Side Effects
{.has-title-underline}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
In mathematics, functions have no side effects.

They will always return the same result given the same parameters.
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```
f(x) = 2x + 1
f(0) = 1
f(1) = 3
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

In software development, such functions are called **pure**.

Yet, not every functions can be pure, like:

 - getting input from the user
 - getting the current time
 - performing a request to an external service
 - ...

Such **impure** functions have **side effects**.

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
**Letlang** provides a mechanism to decouple the handling of a side effect from
a function.

Using the `effect` statement, you can declare a new type of side effect:
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
class log_level(lvl: "debug" | "info");

effect log(log_level, string) -> @ok;
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
Inside a function, you can then trigger the effect, delegating its handling to
the caller.

This is done with the `perform` keyword followed by a call to the effect:
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
func greet(name: string) -> @ok {
  perform log("info", "Hello " <> name);
}
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
Using a `do {}` block and one or more `intercept` clause, the caller can handle
the side effect.

An `intercept` clause uses pattern matching to select which effect to handle.

The last expression of an `intercept` clause will be used as return value of the
`perform` keyword.

Unhandled effects are then propagated to the builtin runtime **Letlang**
includes during compile time.

If the effect is unknown to the runtime, the program will crash with a
stacktrace.
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
module example::main;

import std::io;

pub func main(args: list<string>) -> @ok {
  @ok := do {
    greet("world");
  }
  intercept log("debug", _message) {
    # silenced
    @ok;
  }
  intercept log("info", message) {
    std::io::println(message);
    @ok;
  };

  perform log("debug", "will crash");
}
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

## Exceptions
{.has-title-underline}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
Exceptions are a special kind of effect: **they do not resume**.
This is a form of early return from a function.

Exceptions can be of any type and are raised with the `throw` keyword.

The caller can handle such exceptions with the `catch` clause in a `do {}`
block.

The value returned by the `do {}` block will be the value returned by the
`catch` clause that handled the exception.

> **NB:** Unhandled exceptions (like any other effects) are propagated to the
> runtime environment and crash the program.
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
func main(pub args: list<string>) -> @ok {
  err := do {
    throw (@error, @not_implemented);
    @never_reached;
  }
  catch (@error, reason) {
    reason;
  };

  err = @not_implemented;

  throw @will_crash;

  @ok;
}
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

## Concurrency
{.has-title-underline}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
Functions have [no color](https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/), any function can be run asynchronously or not, it
is up to the caller to decide.

> **WORK IN PROGRESS**

{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
module example::main;

import std::io;

func task() -> @ok {
  # do stuff
  @ok;
}

pub func main(args: list<string>) -> @ok {
  proc_id := spawn task();
  std::io::println("Task PID: " <> proc_id);
  @ok;
}
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

## Message passing
{.has-title-underline}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}

> **WORK IN PROGRESS**

{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
module example::main;

import std::io;

func task(parent: pid) -> @ok;
  send(parent, "hello");
  @ok;
}

pub func main(args: list<string>) -> @ok {
  proc_id := spawn task(self())

  receive {
    message(proc_id, msg) => {
      std::io::println(msg);
    }
    exited(proc_id) => {
      std::io::println("Child exited!");
    }
  };

  @ok;
}
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

## Pipeline operator
{.has-title-underline}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
The pipeline operator is used to chain operations by injecting the value in the
lefthand side as first argument to the function call in the righthand side:
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
x |> add(5) |> mul(2);
# equivalent to
mul(add(x, 5), 2);
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}
