---
title: Compilation Unit
layout: spec
weight: 1
---

# Compilation Unit

A Letlang source file corresponds to a **module**, which is Letlang's compilation
unit.

There are 2 kinds of modules:

 - [library modules](./spec/linkage/compilation-unit#library-modules)
 - [executable modules](./spec/linkage/compilation-unit#executable-modules)

## Library Modules

They MUST be translated to a single Rust library crate.

They MUST expose a [Binary Module Interface](./spec/linkage/binary-module-interface).

The generated Rust library crate SHOULD depend on an `llruntime` Rust crate
containing the Letlang runtime.

## Executable modules

They MUST be translated to a Rust library crate.

They MUST expose a `main` function.

A Rust executable crate containing the Letlang runtime bootstrap code, calling
the `main` function, MUST be generated.

The generated Rust library crates SHOULD depend on an `llruntime` Rust crate
containing the Letlang runtime.
