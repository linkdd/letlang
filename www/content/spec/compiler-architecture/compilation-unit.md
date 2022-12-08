---
title: Compilation Unit
weight: 5
---

Each Letlang module **MUST** correspond to a single source file with the
extension `.let`.

The compiler **MUST** generate a single Rust library crate for each module.

If the Letlang project specify an `[executable]` section, the compiler **MUST**
generate a Rust binary crate, initializing the runtime and calling the `main`
function.

All generated Rust crates **SHOULD** be part of a Cargo workspace in the
`.llbuild` folder.
