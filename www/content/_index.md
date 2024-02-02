---
title: Homepage
---

<div class="[&>p]:flex [&>p]:items-center [&>p]:justify-center [&>p]:gap-3 mb-8">

![status: WIP](https://img.shields.io/badge/status-WIP-red)
![version: 0.0.0](https://img.shields.io/badge/version-v0.0.0-brightgreen)
![license: MIT](https://img.shields.io/badge/license-MIT-blue)

</div>

**Letlang** is a general purpose functional programming language.

Implemented in [Rust](https://www.rust-lang.org), it compiles to Rust, allowing
you to target any platform supported by [LLVM](https://llvm.org).

<article class="m-8 rounded-md shadow-md">
<header class="px-3 py-2 bg-red-600 text-white font-semibold rounded-t-md">Work in progress</header>

<div class="p-6 bg-red-100 text-red-600 rounded-b-md">

**Letlang** is in a very early stage.

The documentation and code examples you may find on this website are **NOT**
definitive and may be subject to change.

</div>
</article>

---

{% include "nav.html" %}

---

<div class="mt-12 grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-x-3 gap-y-8">

<div>
<div class="pb-3 text-3xl text-center font-semibold">Dynamic Type System</div>
<hr class="mx-8" />

```letlang
let even: class[int] {
  (n) -> n % 2 = 0,
};

let odd: class[int & !even];
```

<div class="text-center">

A value does not have a single type, instead the type definition determines what
value it contains.

`42` is a `number`, an `int`, an `even` but not an `odd`.

</div>
</div>

<div>
<div class="pb-3 text-3xl text-center font-semibold">Pattern Matching</div>
<hr class="mx-8" />

```letlang
(@ok, val) := (@ok, 42);

(@ok, res) := (@error, @oops); # error

(a, b) := (1, 2);
```

<div class="text-center">

The `:=` operator is a **pattern matching** operator, used to bind values to new
variables, and extract values from complex data structures.

</div>
</div>

<div>
<div class="pb-3 text-3xl text-center font-semibold">Lazy Constraints</div>
<hr class="mx-8" />

```letlang
let (x: int, y: int) with x > y {
  x := 0;
  y := 1; # error
};
```

<div class="text-center">

`let` expressions create a new scope where future bindings are constrained to a
type and a guard expression.

</div>
</div>

<div>
<div class="pb-3 text-3xl text-center font-semibold">Actor Based Concurrency</div>
<hr class="mx-8" />

```letlang
let task: func[() -> @ok] {
  () -> @ok,
};

let main: func[() -> @ok] {
  () -> {
    (@ok, proc_id) := spawn task();
    # do something
    @ok;
  },
};
```

<div class="text-center">

Run any function in a new **Letlang** *process* (concurrent task), inspired by
[Erlang](https://www.erlang.org)'s design, and backed by [Tokio](https://tokio.rs).

Your program will live as long as there is a running *process* even if the
`main()` function returned.

Every *process* have a unique identifier (`pid`) used for communication and
monitoring.

</div>
</div>

<div>
<div class="pb-3 text-3xl text-center font-semibold">Message Passing</div>
<hr class="mx-8" />

```letlang
(@ok, proc_id) := spawn task(std::proc::self());
# send a signal to this process on exit
std::proc::link(proc_id);

# blocks until signal is received
receive {
  (@message, proc_id, msg) -> {
    # message signal received
  },
  (@exited, proc_id, reason) -> {
    # exit signal received
  },
};
```

```letlang
@ok := std::proc::send(proc_id, "hello");
```

<div class="text-center">

Each *process* has a mailbox where signals can be queued. There are 2 kinds of
signals: **message** and **exited**.

Processes can send messages to each others. When a process exits (or crashes),
an **exited** signal is sent to every linked process.

</div>
</div>

<div>
<div class="pb-3 text-3xl text-center font-semibold">Effect Handlers</div>
<hr class="mx-8" />

```letlang
let log: effect[(string) -> @ok];
```

```letlang
do {
  @ok := perform log("hello");
}
intercept log {
  (msg) -> std::io::println(msg),
};
```

<div class="text-center">

Delegate the handling of side effects to the caller, or let them bubble up to
the **Letlang** runtime.

</div>
</div>

</div>
