---
title: Containers
layout: spec
weight: 5
---

# Containers

Letlang has 3 types of containers:

 - [tuples](./spec/expressions/containers#tuples)
 - [named tuples](./spec/expressions/containers#named-tuples)
 - [lists](./spec/expressions/containers#lists)

## Tuples

### Syntax

{% apply grammkit %}
tuple_expression
  = ("(" (expression ",")? ")")
  / ("(" expression ("," expression)* ","? ")")
{% endapply %}


### Example

```letlang
(1, 2);
```

## Named tuples

Order in a named tuple MUST NOT matter.

### Syntax

{% apply grammkit %}
namedtuple_expression
  = ("{" (namedtuple_pair ",")? "}")
  / ("{" namedtuple_pair ("," namedtuple_pair)* ","? "}")

namedtuple_pair = identifier ":" expression

access_expression = expression "." identifier
{% endapply %}


### Example

```letlang
user := { name: "john", age: 42 };
"john" := user.name;
```

## Lists

### Syntax

{% apply grammkit %}
list_expression
  = ("[" (expression ",")? "]")
  / ("[" expression ("," expression)* ","? "]")

list_headtail_expression
  = "[" expression ("," expression)* ","? "|" expression "]"
{% endapply %}


### Example

```letlang
l := [3, 4, 5];
l := [1, 2 | l];
[1, 2, 3, 4, 5] := l;
```
