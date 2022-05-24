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

Implemented in *[Rust](https://www.rust-lang.org/)* and
*[Python](https://www.python.org)*, it compiles to Rust, allowing you to target
any platform supported by [LLVM](https://llvm.org).
{{< /markdown >}}
{{< /hero >}}

{{< message class="is-danger has-text-justified mx-6" header="Work In Progress" >}}
**Letlang** is opensource but has not yet been published because of its very
early stage.

The documentation and code examples you may find on this website are **NOT**
definitive and may be subject to change.

If you'd like to contribute, feel free to
[contact me by mail](mailto:david.jose.delassus@gmail.com).
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
module "example.main";

import "std";

func main(args: list<string>) -> @ok {
  std.print("Hello World");
  @ok;
}
```
{{< /markdown >}}
{{< /column >}}

{{< /columns >}}

## Working with undefined
{.has-title-underline}

First, let's clarify a few things:

 - there is no `undefined` or `null` value in **Letlang**
 - every variable has a value
 - you don't need to define its value to use the variable

Using the `let` keyword, we can define properties:

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
 - `n` is a number, no specific value defined
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
let n: number;
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
 - `n` still has no value, but it must be a positive number
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
let n > 0;
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
 - ⚠ incompatible with known properties
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
let n < 0;
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

Using the `=` comparison operator and the `solvable {}` block, we can write
equations:

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
- the equation has one or more solutions, so it is `true`
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
let x: number;
solvable { 3 + x = 0 };
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
- the equation has no solution, so it is `false`
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
let x: number;
solvable { 1 / x = 0 };
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
 - if we try to assign the undefined variable to another, a compilation error is
thrown:
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
let x: number;
y := 2 * x + 1;
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

> **NB:** A SAT solver is included in the runtime to solve the equations.

## Expressive Type System
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
module "example.main";

import "std";

class int(n: number) {
  frac(n) = 0;
}

class even(n: int) {
  solvable {
    thereis k: int, n = 2 * k;
  };
}

class odd(n: int & !even);

class vector(xy: (number, number));
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

Using the above definitions, we can assert:

```letlang
0.3 is int;    # false
42 is number;  # true
42 is int;     # true
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

Type parameters can also be constrained:

```letlang
class blittable(v: number | boolean | atom);

func dump<T: blittable>(v: T) -> @ok {
  # do stuff
}

dump::<string>("won't compile")
```

## Recursion and Tail Call Optimization
{.has-title-underline}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
A recursive function is a function which calls itself.

If the self call is the last instruction in the function's body, it is called a
**tail call**.

Such functions can be *unrolled* into a simple loop, this is called
[Tail Call Optimization](https://www.youtube.com/watch?v=-PX0BV9hGZY):
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
func count<T>(acc: number, l: list<T>) -> int {
  match l {
    [] => acc,
    list<T> => {
      [_head | tail] := l;
      count(acc + 1, tail);
    },
  };
}

count(0, [1, 2, 3, 4, 5]) = 5;
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

## Coroutines and streams
{.has-title-underline}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
Functions have [no color](https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/), any function can be run asynchronously or not.

The keyword `coro` followed by a function call will run the function
asynchronously and return a **coroutine value**.

The keyword `join` takes a **coroutine value** as argument and will block until
the coroutine is finished and finally returns the coroutine's result.

This allows the program to work on something else while the coroutine's value is
being computed, and introduce a synchronization point later on when the value is
needed.
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
module "example.main";

import "std";

func count(msg: string) -> string {
  std.strlen(msg);
}

func main(args: list<string>) -> @ok {
  c := coro echo("foo");
  # do other stuff
  let 3 = join c;

  @ok;
}
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
Streams can then be used for communication between coroutines.

A stream is a special object for bidirectionnal Input/Output of values.

Write are instantaneous but reads will block until a value has been written.
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
let s: stream<int>;
let v: int;

# write a value to the stream
s |<< 42;

# read a value from the stream to a variable
s |>> v;
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

Streams used with coroutines offer a simple synchronization method between
parallel tasks:

```letlang
module "example.main";

import "std";

func double(s: stream<int>) -> @ok {
  let x: int;

  s |>> x;

  match x {
    -1 => @ok,
    int => {
      s |<< (x * 2);
      double(s);
    }
  };
}

func main(args: list<string>) -> @ok {
  let s: stream<int>;

  c := coro double(s);

  s |<< 1 |<< 2 |<< 3 |<< 4 |<< -1;

  let @ok = join c;

  let (a, b, c, d): (int, int, int, int);

  s |>> a |>> b |>> c |>> d;
  let a = 2;
  let b = 4;
  let c = 6;
  let d = 8;

  @ok;
}
```

## Algebraic Effects
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
  res := perform log("info", "Hello ${name}");
  res;
}
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
Using a `do {}` block and one or more `intercept` clause, the caller can handle the
side effect.

An `intercept` clause uses pattern matching to select which effect to handle.

The last expression of an `intercept` clause will be used as return value of the
`perform` keyword.

> **NB:** Types are also checked at runtime to ensure type safety.

Unhandled effects are then propagated to the builtin runtime **Letlang**
includes during compile time.

If the effect is unknown to the runtime, the program will crash with a
stacktrace.
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
module "example.main";

import "std";

func main(args: list<string>) -> @ok {
  let @ok = do {
    greet("world");
  }
  intercept log("debug", _message) {
    # silenced
    @ok;
  }
  intercept log("info", message) {
    std.print(message);
    @ok;
  };

  perform log("fatal", "won't compile");
  perform log("debug", "will crash");
}
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

The builtin runtime provides a few effect handlers out of the box:

```letlang
effect gettime() -> number;

effect println(string, @stdout | @stderr) -> @ok;
effect readline() -> std.result<string, std.errno>;

class fmode(m: @w | @wb | @r | @rb | @a);
effect fopen(string, fmode) -> std.result<std.file, std.errno>;
effect fwrite(std.file, string) -> std.result<(), std.errno>;
effect fread(std.file, number) -> std.result<string, std.errno>;
effect fclose(std.file) -> std.result<(), std.errno>;

effect sockbind(std.socket) -> std.result<(), std.errno>;
effect sockaccept(std.socket) -> std.result<std.socket, std.errno>;
effect socksend(std.socket, string) -> std.result<number, std.errno>;
effect sockread(std.socket, number) -> std.result<string, std.errno>;
effect sockclose(std.socket) -> std.result<(), std.errno>;
```

Exceptions are a special kind of effect: **they do not resume**.
This is a form of early return from a function.

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
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
func main(args: list<string>) -> @ok {
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

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
A `do{}` block can have a `finally` clause to execute code afterwards.

This is useful to free resources allocated within the function in case of error.

The `finally` clause will be executed after all other clauses are done.
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
func will_fail() -> @ok {
  resource := create_resource();

  do {
    throw @did_fail;
  }
  finally {
    delete_resource(resource);
  };

  @ok;
}
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

## Set Builder Notation
{.has-title-underline}

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
**Letlang** has infinite sets. They are described using a variable and a
predicate indicating which values are included:
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
Sa := { x: number | x > 0 };
Sb := { x: Sa | x < 10 };
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

The above definitions leads to the following equalities:

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
```letlang
Sa is set<number>
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
Sb is set<number>;
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
42 in Sa;
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
42 in Sb;
```
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
 - this is `false`
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

> **NB:** There is no infinite lists, because you can't always define a
> beginning.

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
Consider the definition on the right, what would be the value of `l[0]`?
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
l := [ x: number | x != 42 ];
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

**About [Russel's paradox](https://en.wikipedia.org/wiki/Russell%27s_paradox):**

`set` is a generic types, they can only contain elements of the underlying type.
Therefore, you cannot build the set of all sets.

If you could build such a set, it would lead to a conflicting definition such
as:

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
 - The set of all sets that do not contain themselves
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
S := { x: set | x not in x };
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

Does the following expression `S in S` returns `true` or `false`?

 - if `S not in S`, then it would match the predicate `x not in x`, implying `S in S`
 - if `S in S`, then it would not match the predicate `x not in x`, implying `S not in S`

This is why the type `set` do not exists, only `set<T>`.

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
`x` is a set that can only contain numbers, not sets of numbers, therefore
`x not in x` would lead to a type mismatch error:

 - `number` in `set<number>` is valid
 - `set<number>` in `set<number>` is not
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
```letlang
# this will lead to a compilation error
S := { x: set<number> | x not in x };
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

## Logical operators
{.has-title-underline}

On top of the common boolean operators `not`, `and`, `or`, **Letlang**
introduces the following ones:

 - `==>`: implication operator
 - `<==>`: biconditional operator

Here is their truthtable:

| `P` | `Q` | `P ==> Q` | `P <==> Q` |
| --- | --- | --------- | ---------- |
| `true` | `true` | `true` | `true` |
| `true` | `false` | `false` | `false` |
| `false` | `true` | `true` | `false` |
| `false` | `false` | `true` | `true` |
{.table .is-bordered .is-hoverable .has-text-centered}
