---
title: Actor-based concurrency
layout: spec
weight: 3
---

# Actor-based concurrency

The `main` function of a Letlang executable MUST be run in a Letlang process.

A Letlang process SHOULD be a lightweight userland thread, also called
"greenthread".

Each Letlang process MUST have a unique identifier and a "mailbox" to receive
signals from other process.

A Letlang executable MUST keep running as long as there is at least one process
running, even if the process associated to the `main` function terminated.

A Letlang "source" process MUST be able to link/unlink itself to/from another
"target" process. When such such link is created, the "source" process will
receive a signal whenever the "target" process terminates (successfully or not).

When exceptions bubble up to the runtime, the MUST terminate the process. As a
consequence, the runtime MUST send an "exited" signal to all linked processes
with the exception as a signal payload.

When effects bubble up to the runtime, if the runtime has no builtin handler for
the effect, the runtime MUST terminate the process. As a consequence, the
runtime MUST send an "exited" signal to all linked processes. In such case, the
signal's payload is implementation-defined.

The function associated to a process MUST return `@ok` to be considered
successful. If the function returns `@ok`, the runtime MUST send an "exited"
signal to all linked processes with an empty payload. If the function returns
any other value, the runtime MUST send an "exited" signal to all linked
processes. In such case, the signal's payload is implementation-defined.

The runtime MUST provide a function to send "message" signals to another
process.
