---
title: 3.3. Side Effects
description: Overview of effect delegation and handling mechanism
prev: /book/chapter-3/2-loops
next: /book/chapter-3/4-exceptions
---

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
