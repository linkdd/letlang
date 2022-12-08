---
title: Toolchain
weight: 6
---

In the `[toolchain]` section of the Letlang project configuration file:

 - the `letlang` field correspond to the version of the `letlang_runtime` Rust crate
 - the `rust` field correspond to the Rust edition to use for the compilation of the generated Rust crates

The compiler **MUST** add the `letlang_runtime` Rust crate as a dependency to
every generated Rust crate, under the name `llcore_runtime`.

The compiler **MUST** scan the module imports to generate the dependencies of
each generated Rust crate.
