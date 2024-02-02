---
title: Operators
layout: spec
weight: 1
---

# Operators

## Syntax

{% apply grammkit %}
logic_binary_operator = "or" / "and"

logic_unary_operator = "not"

bitwise_arithmetic_binary_operator = "|" / "&" / "^"

bitwise_arithmetic_unary_operator = "~"

inclusion_binary_operator = "not"? "in"

comparison_binary_operator = "<" / "<=" / "=" / "!=" / ">=" / ">"

bitwise_shift_binary_operator = "<<" / ">>"

concatenation_binary_operator
  = string_concatenation_operator
  / list_concatenation_operator

string_concatenation_operator = "<>"

list_concatenation_operator = "++"

arithmetic_binary_operator = "+" / "-" / "*" / "/" / "%" / "**"

arithmetic_unary_operator = "-"
{% endapply %}


## Precedence Table

> **NB:** Higher precedence have priority.

| Operator | Precedence | Associativity |
| --- | --- | --- |
| `a := b` | 1 | right to left |
| `a is b` | 2 | left to right |
| `a is not b` | ^^ | ^^ |
| `a |> b()` | 3 | ^^ |
| `throw b` | 4 | right to left |
| `a or b` | 5 | left to right |
| `a and b` | 6 | ^^ |
| `a | b` | 7 | ^^ |
| `a ^ b` | 8 | ^^ |
| `a & b` | 9 | ^^ |
| `a in b` | 10 | ^^ |
| `a not in b` | ^^ | ^^ |
| `a = b` | 11 | ^^ |
| `a != b` | ^^ | ^^ |
| `a < b` | 12 | ^^ |
| `a <= b` | ^^ | ^^ |
| `a >= b` | ^^ | ^^ |
| `a > b` | ^^ | ^^ |
| `a << b` | 13 | ^^ |
| `a >> b` | ^^ | ^^ |
| `a <> b` | 14 | ^^ |
| `a ++ b` | 15 | ^^ |
| `a + b` | 16 | ^^ |
| `a - b` | ^^ | ^^ |
| `a * b` | 17 | ^^ |
| `a / b` | ^^ | ^^ |
| `a % b` | ^^ | ^^ |
| `a ** b` | 18 | ^^ |
| `-a` | 19 | right to left |
| `not a` | ^^ | ^^ |
| `~a` | ^^ | ^^ |
| `a.b` | 20 | left to right |
