---
title: Language Target
weight: 4
---

The Letlang Abstract Syntax Tree **MUST** be translated to Rust code.

Once the AST has been validated by a semantic walk, the generated code
**SHOULD** be valid and not produce compilation errors.

If for some reason, the previous requirement cannot be satisfied, the generated
code **SHOULD** include comments that will appear in the Rust compiler's error
report.

Those comments **SHOULD** be used to pin-point the error back to the Letlang
source file.

Example:

```rust
foo(); // file="example/foo.let" line=8 column=4
```
