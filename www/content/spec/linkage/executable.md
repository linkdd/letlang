---
title: Executable
layout: spec
weight: 4
---

# Executable

When building an executable from the Letlang module `dummy.let`, a Cargo project
named `lldep_dummy` SHOULD be created in the `.lltarget` folder. It SHOULD
contain the following files:

```text
.
├── libdummy.lli        -- Binary Module Interface
└── lldep_dummy         -- Rust crate name corresponding to Letlang module
    ├── Cargo.toml      -- Cargo project manifest
    └── src
        ├── lib.rs      -- Letlang module translated to Rust library crate
        └── main.rs     -- Letlang runtime bootstrap code, calling the main function
```

When building an executable `dummy` which can depend on modules `foo` and `bar`
with:

```shell
$ letlangc --type=exe dummy.let -L. -lfoo -lbar
```

The compiler MUST look for `libfoo.lla` (as specified by `-lfoo`) and
`libbar.lla` (as specified by `-lbar`) in the current directory (as specified
by `-L.`).

The archives SHOULD be extracted in a `.lltarget` folder where it can be used by
the compiler:

 - to read the binary module interface when resolving symbols
 - to specify the correct dependencies in the `lldep_dummy` Cargo project manifest
 - to build the whole project

The `.lltarget` folder MAY be a Cargo workspace.

The final executable `dummy` (or `dummy.exe` on Windows) MUST be placed in the
current directory.

After compilation, the `.lltarget` folder MAY be removed.
