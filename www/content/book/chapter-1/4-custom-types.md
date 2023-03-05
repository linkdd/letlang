---
title: 1.4. Custom Types
description: Build your own data types
prev: /book/chapter-1/2-containers
next: /book/chapter-2
---

# Proper classes

In set theory, a **[proper class](https://en.wikipedia.org/wiki/Class_(set_theory))**
is a collection of objects that is not a set, and therefore cannot be contained
by another entity.

**Letlang** types are proper classes.

A class is defined by:

 - zero or more type parameters
 - one constructor parameter
 - optionally a predicate that each value must validate

The constructor parameter determines the structure of the values contained in
the class.

# Examples

```letlang
class even(n: int) {
  n % 2 = 0;
}

class odd(n: int & !even);
```

```letlang
class vector(v: {x: number, y: number});

class unit_vector(v: vector) {
  v.x ** 2 + v.y ** 2 = 1;  # sqrt() not needed: 1**2 = 1
}
```

# Generics

Using generics, the Result type can be constructed:

```letlang
class ok<T>(r: (@ok, T));
class err<E>(r: (@error, E));
class result<T, E>(r: ok<T> | err<E>);
```

```letlang
(@ok, 42) is ok<int>;               # true
(@ok, 42) is err<string>;           # false
(@ok, 42) is result<int, string>;   # true

(@error, "invalid") is ok<int>;             # false
(@error, "invalid") is err<string>;         # true
(@error, "invalid") is result<int, string>; # true
```
