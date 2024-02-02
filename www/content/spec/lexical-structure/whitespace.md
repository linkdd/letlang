---
title: Whitespace
layout: spec
weight: 5
---

# Whitespace

The lexer MUST skip whitespaces between lexical tokens.

### Syntax

{% apply grammkit %}
whitespace = [ \t\r\n\f]+
{% endapply %}
