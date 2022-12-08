---
title: Modules
weight: 1
---

Every Letlang module (which correspond to a source file with the extension
`.let`) **MUST** begin with a `<module-declaration-statement>` rule.

```bnf
<module-declaration-statement> :=
  "module" <module-path> ";"
  ;

<module-path> :=
  <identifier> ("::" <identifier>)*
  ;

<identifier> :=
  /[_a-zA-Z][_0-9a-zA-Z]*/
  ;
```

Examples:

```letlang
module std::io;
```

```letlang
module foo::main;
```

Every module path **MUST** be unique.
