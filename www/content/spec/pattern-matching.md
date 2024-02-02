---
title: Pattern Matching
layout: spec
weight: 6
---

# Pattern Matching

Pattern matching is used to branch on a pattern or destructure data.

Many expressions have a set of `pattern_matching_clause` rules used for
branching.

## Branching

### Syntax

{% apply grammkit %}
pattern_matching_clause = clause_patterns clause_guard? "->" clause_body

clause_patterns = "(" (pattern ("," pattern)* ","? )? ")"

clause_guard = "when" expression

clause_body
  = expression
  / ("{" (expression ";")+ "}")
{% endapply %}


### Example

```letlang
let natural: class[int] {
  (n) -> n >= 0,
};

let collatz: func[(natural) -> natural] {
  (n) when n % 2 = 0 -> n / 2,
  (n) -> 3 * n + 1,
};
```

### Semantics

In a class constraint, if no clause matches, then the object MUST NOT belong to
the class:

```letlang
let natural: class[int] {
  (n) when n >= 0 -> true,
};

# is equivalent to:

let natural: class[int] {
  (n) when n >= 0 -> true,
  (_) -> false,
};
```

In all other cases, if no clause matches, an exception MUST be thrown, the value
of this exception is defined by the implementation:

```letlang
let foo: func[(int) -> @ok] {
  (1) -> @ok,
  (2) -> @ok,
};

# is equivalent to:

let foo: func[(int) -> @ok] {
  (1) -> @ok,
  (2) -> @ok,
  (_) -> throw @implementation_defined_error,
};
```

Each clause MUST create a new scope.

A clause MUST evaluate to the value the last expression in its branch evaluates
to.

The guard expression of a clause MUST evaluate to a boolean (`true` or `false`).

If an exception is thrown in a guard expression, it SHOULD be ignored and be
equivalent to returning `false`.

## Pattern syntax

{% apply grammkit %}
pattern
  = pattern_ignore
  / pattern_binding
  / pattern_literal
  / pattern_tuple
  / pattern_namedtuple
  / pattern_list
  / pattern_list_headtail
  / pattern_eval

pattern_ignore = "_"

pattern_binding = identifier

pattern_literal = atom / bool / number / string

pattern_tuple
  = ("(" ("..." ","?)? ")")
  / ("(" pattern ("," pattern)* ("," "...")? ","? ")")

pattern_namedtuple
  = ("{" ("..." ","?)? "}")
  / ("{" pattern_namedtuple_pair ("," pattern_namedtuple_pair)* ("," "...")? ","? "}")

pattern_namedtuple_pair = identifier ":" pattern

pattern_list
  = ("[" ("..." ","?)? "]")
  / ("[" pattern ("," pattern)* ("," "...")? ","? "]")

pattern_list_headtail
  = "[" pattern ("," pattern)* ","? "|" pattern "]"

pattern_eval
  = "${" expression "}"
{% endapply %}


### Semantics

The ellipsis `...` means *"and the rest"*:

```letlang
# a = 1, b = 2
[a, b, ...] := [1, 2, 3, 4];

# a = 1, b = 2, tail = [3, 4]
[a, b | tail] := [1, 2, 3, 4];

# this will throw an error
[a, b] := [1, 2, 3, 4];
```

Binding a value to a name MUST happen immediatly:

```letlang
# a = 2, because it is rebound a second time
(a, a) := (1, 2);

# a = 1 and can be used immediatly in an `pattern_eval` rule
(a, ${a + 1}) := (1, 2);

# will throw an error, because a = 1, and 1 does not match 2
(a, ${a}) := (1, 2);
```
