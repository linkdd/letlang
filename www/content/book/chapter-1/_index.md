---
title: 1. Type System
description: Overview of the Letlang type system
prev: /book/
next: /book/chapter-1/1-primitive-types
---

# Introduction

The **Letlang** type system is entirely dynamic.

In mathematics, a value does not have a single type. Instead, it belongs to one
or more sets:

 - `42` is an integer, a real number, a scalar, ...
 - `(2, 1)` is a 2D vector, a 2x1 matrix, ...

This concept is at the core of **Letlang**'s type system.

Each type defines a structure, and optionnaly a predicate, indicating what
values belong to the type.

We can consider a type as a function that takes a value as parameter, and
returns `true` or `false` whether that value belongs to the type or not.
