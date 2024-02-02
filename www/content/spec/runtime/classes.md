---
title: Builtin classes
layout: spec
weight: 1
---

# Builtin classes

The runtime MUST provide the following classes:

| Class | Description |
| --- | --- |
| `term` | Contains everything, all other classes are subsets of this one |
| `atom` | Contains all atoms (ie. `@ok`, `@error`, ...) |
| `bool` | Contains `true` and `false` |
| `number` | Contains all numbers (ie. `1`, `2.3`, ...) |
| `int` | Contains all integers, it is a subset of `number` |
| `string` | Contains all strings (ie. `"hello world"`) |
| `list<T>` | Contains all lists where their elements are of class `T` |
| `proplist<T>` | Contains all lists where their elements are of class `(atom, T)` |
| `pid` | Opaque class that contains all process identifiers |
| `iocap` | Opaque class that contains all IO capabilities |
{.table-hoverable .table-col0-minw .table-col1-left}

Each Letlang value is also a singleton type, the runtime MUST provide an
implementation for such types.

The runtime MUST provide an implementation for tuple types and named tuple types.
