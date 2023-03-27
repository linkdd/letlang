---
title: Modules
weight: 1
---

Every Letlang module (which correspond to a source file with the extension
`.let`) **MUST** begin with a `module_decl_statement` rule.

{{< grammar-block "module" >}}

Examples:

```letlang
module std::io;
```

```letlang
module foo::main;
```

Every module path **MUST** be unique.
