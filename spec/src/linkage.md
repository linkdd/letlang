# Linkage

**DRAFT**

## Abstract

```shell
letlangc --type=lib foo.let
letlangc --type=lib bar.let -L. -lfoo
letlangc --type=exe main.let -L. -lfoo -lbar
```

## Archive Format

The following command Will produce an **AR** (unix archive) file named
`libfoo.lla`:

```shell
letlangc --type=lib foo.let
```

This archive will contain the following files:

 - `libfoo.lli`: contains the exported sysmbols of the module as JSON data,
   the compiler will use this information when resolving names
 - `libfoo.rlib`: this is the built Rust library crate

## Linking

The following command will expect the file `libfoo.lla` to be present in the `.`
directory:

```shell
letlangc --type=lib bar.let -L. -lfoo
```

The compiler will extract the archive into `./.lltarget/` (or any existing path
specified by the `-b` option).

The file `./.lltarget/libfoo.lli` will be used for name resolution during the
compilation, and the file `./.lltarget/libfoo.rlib` will be used when calling
`rustc`.

The result will be another archive `libbar.lla`.

The following command will work exactly like before, but will produce an
executable instead:

```shell
letlangc --type=exe main.let -L. -lfoo -lbar
```

Once built, the `./.lltarget` folder can be removed.
