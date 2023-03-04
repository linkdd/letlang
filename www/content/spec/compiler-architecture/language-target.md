---
title: Language Target
weight: 4
---

The Letlang Abstract Syntax Tree **MUST** be translated to Rust code.

Generated code **MUST** include source-mapping comments:

```rust
//source-map:begin=NodeType file="example/foo.let" line=8 column=4
foo();
//source-map:end=NodeType
```

Once the AST has been validated by a semantic walk, the generated code
**SHOULD** be valid and not produce compilation errors.

If for some reason, the previous requirement cannot be satisfied, the source
mapping information **MUST** be used to pin-point the error back to the Letlang
source file.
