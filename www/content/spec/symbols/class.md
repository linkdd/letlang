---
title: Class
layout: spec
weight: 1
---

# Class

A class is a collection of objects, similar to **proper classes** in Set theory.

Any object satisfying the constraints of a class belongs to it. Therefore, an
object does not have a single type.

A class is defined structurally from a type expression, and can optionally have
constraint expressions to verify if the object satisfy the desired properties.

### Syntax

{% apply grammkit %}
class_definition
  = "class" "[" type_expression "]"
  ("{" pattern_matching_clause ("," pattern_matching_clause)* ","? "}")?

type_expression
  = (type_expression "&" type_expression)
  / (type_expression "|" type_expression)
  / ("!" type_expression)
  / ("(" type_expression ")")
  / type_term

type_term
  = type_ref
  / type_literal
  / type_tuple
  / type_namedtuple

type_ref = symbol_path ("<" type_expression ("," type_expression)* ","? ">")?

type_literal = atom / bool / number / string

type_tuple
  = ( "(" ")" )
  / ( "(" type_expression "," ")" )
  / ( "(" type_expression ("," type_expression)+ ","? ")" )

type_namedtuple
  = ( "{" "}" )
  / ( "{" type_namedtuple_pair ("," type_namedtuple_pair)* ","? "}" )

type_namedtuple_pair = identifier ":" type_expression
{% endapply %}


### Examples

```letlang
let even: class[int] {
  (n) -> n % 2 = 0,
};

let odd: class[int & !even];
```

```letlang
let ok<t>: class[(@ok, t)];
let err<e>: class[(@error, e)];
let result<t, e>: class[ok<t> | err<e>];
```

### Semantics

Each clause of a class MUST evaluate to a boolean (`true` or `false`).

If an exception is thrown in the body of a clause, it SHOULD be ignored and be
equivalent to returning `false`.
