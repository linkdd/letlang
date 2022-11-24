---
title: 4.2. Processes
description: Introduction to Letlang's concurrency model
prev: /book/chapter-4/1-functions
next: /book/chapter-4/3-message-passing
---

# Introduction

A Letlang process is a green-thread running a single function to completion (or
until it crashes). Every process has a unique identifier (a `pid`).

Functions have [no color](https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/), any function can be run in a process. It is up to
the caller to decide if a function should be run in a new process or not.

When spawning a new process, its *pid* is returned, allowing the caller to
interact with it.

**Example:**

```letlang
func task() -> @ok {
  # do stuff
  @ok;
}

func main() -> @ok {
  (@ok, proc_id) := spawn task();

  # do stuff

  @ok;
}
```

# About side effects and exceptions

In every process, if a side effect (or an exception) is not intercepted/caught
with a `do{}` expression (see [chapter 3](/book/chapter-3)), it bubbles up to
the Letlang runtime.

**Example:**

```letlang
func task() -> @ok {
  throw @oops;
}

func main() -> @ok {
  @ok := do {
    task();  # the exception is caught
  }
  catch @oops {
    @ok;
  };

  spawn task();  # the exception will bubble up to the runtime

  # do stuff

  @ok;
}
```

When an exception or an unknown effect bubbles up to the runtime, the process
crashes, **but not the whole program**.

# Implementation details

When starting, Letlang starts a *node* which listens for commands:

 - spawning processes
 - linking/unlinking processes
 - sending message to a process
 - notification of process termination

The node is implemented with the Rust crate [tokio](https://tokio.rs).

When the node spawns a process, it creates a new PID and a mailbox to exchange
signals. Then it starts 2 asynchronous tasks as follow:

```rust
tokio::spawn(async move {
  let process_handle = tokio::spawn(async move {
    // run process's function
  });

  let res = process_handle.await;
  // notify the node of the process termination
});
```

If the process terminates normally, or panics, it is caught and the node is
notified.

> **NB:** The `main` function is automatically spawned as a new process (the
> first one).

When every process are terminated, the node stops and the program terminates.
This means that the `main` function can terminate before the tasks it spawned,
it won't stop the program.
