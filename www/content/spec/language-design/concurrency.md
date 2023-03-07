---
title: Concurrency Model
weight: 7
---

# Processes

A Letlang process **MUST** have a unique identifier.

The process identifier **MUST** belong to the type `pid`.

Letlang processes **MUST** execute concurrently.

# Message passing

Letlang processes can send signals to each-other.

Every Letlang process **MUST** have a mailbox to receive signals.

Signals are either:

 - `message`
 - `exited`

Every signal **MUST** include the PID of the process who sent the signal.

Every signal **MUST** include a value:

 - `message`: the value **MUST** be the data sent by the other process
 - `exited`: the value **MUST** be either `@ok` if the process terminated normally, or `(@error, string)` if the process crashed

A process can block until it received a signal or after a timeout period.

The timeout period **MUST** be in milliseconds.

**Syntax:**

```bnf
<expression-term> :=
  | ...
  | <receive-block>
  | ...
  ;

<receive-block> :=
  "receive" "{"
  <receive-signal-branch> ("," <receive-signal-branch>)*
  "}"
  [ "after" <expression> "{" <proposition>+ "}" ]
  ;

<receive-signal-branch> :=
  | <receive-signal-branch-simple>
  | <receive-signal-branch-block>
  ;

<receive-signal-branch-simple> :=
  <receive-signal-branch-type> "(" <pattern> "," <pattern> ")" "=>" <expression>
  ;

<receive-signal-branch-block> :=
  <receive-signal-branch-type> "(" <pattern> "," <pattern> ")" "=>"
  "{" <proposition>+ "}"
  ;

<receive-signal-branch-type> :=
  | "message"
  | "exited"
  ;
```

**Example:**

```letlang
receive {
  message(from, data) => {
    # do something with data
  },
  exited(source, reason) => {
    # do something with reason
  },
}
after 5000 {
  # do something
};
```

A `receive{}` block **MUST** evaluate to the value returned by the executed branch:

 - if a `message` signal was received, it evaluates to the matching branch
 - if an `exited` signal was received, it evaluates to the matching branch
 - if no signal was receive within the timeout period, it evaluates to the `after` branch

# Links between processes

Letlang processes can be linked together.

When a process terminates, all linked processes **MUST** receive an `exited`
signal.

# Spawning a new process

Every function can be called as a new process.

Spawning a new process **MUST** return either:

 - `(@ok, pid)`: where `pid` is the PID of the new process
 - `(@error, string): where `string` is the reason why the process failed to be spawned

**Syntax:**

```bnf
<expression-term> :=
  | ...
  | <spawn-call>
  | ...
  ;

<spawn-call> :=
  "spawn" <expression> "(" <expression> ("," <expression>)* ")"
  ;
```

**Example:**

```letlang
(@ok, proc) := spawn some_function();
```

# Termination

A Letlang program **MUST** terminate when all processes have terminated.
