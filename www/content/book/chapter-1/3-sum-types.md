---
title: 1.3. Sum Types
description: Description of Letlang's algebraic data types
prev: /book/chapter-1/2-containers
next: /book/chapter-1/4-custom-types
---

# Negation

Types can be inverted with the operator `!`, meaning: **not of this type**:

```letlang
42 is !string;    # true
42 is !number;    # false
2.3 is !int;      # true
```

# Union

Types can be composed with the operator `|`, meaning: **one of either types**:

```letlang
42 is (int | string);       # true
"hello" is (int | string);  # true
@ok is (int | string);      # false
```

This allows you to build lists of more heterogenous types:

```letlang
[42, "hello"] is list<int | string>;  # true
[42, "hello"] is list<int>            # false
```

# Intersection

Types can be composed with the operator `&`, meaning: **all of each type**:

```letlang
42 is (number & !int);  # false
2.3 is (number & !int); # true
```
