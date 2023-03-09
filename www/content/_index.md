---
title: Homepage

sources:
  - https://medium.com/feed/tag/letlang
categories: ["letlang"]
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

{{< columns class="is-vcentered my-6" >}}
{{< column class="is-one-third" >}}
{{< center >}}
{{< action href="/book/" class="is-info p-6" >}}
<i class="fas fa-book"></i>
<b>Handbook</b>
{{< /action >}}
{{< /center >}}
{{< /column >}}

{{< column class="is-one-third" >}}
{{< center >}}
{{< action href="/spec/" class="is-warning p-6" >}}
<i class="fas fa-code"></i>
<b>Language Specification</b>
{{< /action >}}
{{< /center >}}
{{< /column >}}

{{< column class="is-one-third" >}}
{{< center >}}
{{< action href="https://github.com/linkdd/letlang" class="is-success p-6" >}}
<i class="fab fa-github"></i>
<b>Download</b>
{{< /action >}}
{{< /center >}}
{{< /column >}}
{{< /columns >}}

---

{{< columns class="my-6" >}}
{{< column class="is-one-third" >}}
{{< markdown >}}
{{< center >}}
# Dynamic Type System
{{< /center >}}

---

```letlang
class even(n: int) {
  n % 2 = 0;
}

class odd(n: int & !even);
```

{{< center >}}
A value does not have a single type, instead the type definition determines what
value it contains.

`42` is a `number`, an `int`, an `even` but not an `odd`.
{{< /center >}}

{{< /markdown >}}
{{< /column >}}

{{< column class="is-one-third" >}}
{{< markdown >}}
{{< center >}}
# Pattern Matching
{{< /center >}}

---

```letlang
(@ok, val) := (@ok, 42);

(@ok, res) := (@error, @oops); # error

(a, b) := (1, 2);
```

{{< center >}}
The `:=` operator is a **pattern matching** operator, used to bind values to new
variables, and extract values from complex data structures.
{{< /center >}}

{{< /markdown >}}
{{< /column >}}

{{< column class="is-one-third" >}}
{{< markdown >}}
{{< center >}}
# Side Effects
{{< /center >}}

---

```letlang
effect log(msg: string) -> @ok;
```

```letlang
do {
  @ok := perform log("hello");
}
intercept log(msg) {
  std::io::println(msg);
};
```

{{< center >}}
Delegate the handling of side effects to the caller, or let them bubble up to
the **Letlang** runtime.
{{< /center >}}


{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

{{< columns class="my-6" >}}
{{< column class="is-one-third" >}}
{{< markdown >}}
{{< center >}}
# Actor Based Concurrency
{{< /center >}}

---

```letlang
func task() -> @ok {
  # do something
  @ok;
}

func main() -> @ok {
  (@ok, proc_id) := spawn task();
  # do something
  @ok;
}
```

{{< center >}}
Run any function in a new **Letlang** *process* (concurrent task), inspired by
[Erlang](https://www.erlang.org/)'s design, and backed by
[Tokio](https://tokio.rs).

Your program will live as long as there is a running *process*, even if the
`main()` function returned.

Every *process* have a unique identifier (`pid`) used for communication and
monitoring.
{{< /center >}}

{{< /markdown >}}
{{< /column >}}

{{< column class="is-one-third" >}}
{{< markdown >}}
{{< center >}}
# Message Passing
{{< /center >}}

---

```letlang
(@ok, proc_id) := spawn task(std::proc::self());
# send a signal to this process on exit
std::proc::link(proc_id);

receive { # blocks until signal is received
  message(proc_id, msg) => {
    # message signal received
  },
  exited(proc_id, reason) => {
    # exit signal received
  },
};
```

```letlang
@ok := std::proc::send(proc_id, "hello");
```

{{< center >}}
Each *process* have a mailbox where signals can be queued. There exist 2 kinds
of signal: **message** and **exited**.

Processes can send messages to each others. When a process exits (or crashes),
an **exited** signal is sent to every linked process.
{{< /center >}}

{{< /markdown >}}
{{< /column >}}

{{< column class="is-one-third" >}}
{{< markdown >}}
{{< center >}}
# Immutable & No GC
{{< /center >}}

---

```letlang
a := 1;
# rebinding to a new value
# does not mutate the previous value
a := 2;
```

```letlang
a := 1;

do {
  a := 2;
  # this binding goes out of scope
};

# a equals 1
```

{{< center >}}
Variables are bound to values during the lifetime of a scope. They can be
rebound to new values, but the previous value is not mutated. There is no
mutable state, let alone **shared** mutable state.

**Letlang** relies on Rust's ownership semantics to free the memory when objects
go out of scope, making it free of any Garbage Collector.
{{< /center >}}

{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

---

{{< newsFeed >}}
