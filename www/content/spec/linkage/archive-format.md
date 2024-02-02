---
title: Archive format
layout: spec
weight: 3
---

# Archive format

Library modules MUST be composed of a Rust library crate and a Letlang binary
module interface.

The Rust crate SHOULD be a Cargo project.

The generated files MUST be distributed in an archive with the `.lla` extension.

The archive SHOULD be created with the `ar` UNIX format.

## Example

When building a module `foo` with:

```shell
$ letlangc --type=lib foo.let
```

The archive `libfoo.lla` MUST be created. It SHOULD contain the following files:

```text
.
├── libfoo.lli        -- Binary Module Interface
└── lldep_foo         -- Rust crate name corresponding to Letlang module
    ├── Cargo.toml    -- Cargo project manifest
    └── src
        └── lib.rs    -- Letlang module translated to Rust library crate
```

## Linking with other modules

When building a module `bar` that depends on `foo` with:

```shell
$ letlangc --type=lib bar.let -L. -lfoo
```

The compiler MUST look for `libfoo.lla` (as specified by `-lfoo`) in the current
directory (as specified by `-L.`).

The archive SHOULD be extracted in a `.lltarget` folder where it can be used by
the compiler:

 - to read the binary module interface when resolving symbols
 - to specify the correct dependencies in the `lldep_bar` Cargo project manifest
 - to test the compilation of the generated Rust code

The `.lltarget` folder MAY be a Cargo workspace.

When building a module `baz` that depends on `bar`, the user MUST specify all
the dependencies (direct and transitive):

```shell
$ letlangc --type=lib baz.let -L. -lfoo -lbar
```

The order of the `-l` options MUST NOT matter.

After compilation, the `.lltarget` folder MAY be removed.
