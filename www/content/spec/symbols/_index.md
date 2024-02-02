---
title: Symbols
layout: spec
weight: 5
---

# Symbols

Symbols can be private or public.

Public symbols compose the interface of a Letlang module.

Symbols can have type parameters.

### Syntax

{% apply grammkit %}
symbol_declaration
  = "let" "pub"? identifier type_parameters? ":"
  (class_definition / function_definition / effect_definition) ";"

type_parameters = "<" identifier ("," identifier)* ","? ">"
{% endapply %}


### Semantic

A symbol declaration MUST create a new scope.

Type parameters MUST define class symbols in the current declaration's scope.
