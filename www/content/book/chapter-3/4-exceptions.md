---
title: 3.4. Exceptions
description: Overview of exception mechanism
prev: /book/chapter-3/3-side-effects
next: /book/chapter-3/5-pipeline-operator
---

# Introduction

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
> runtime environment and crash the process.
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
