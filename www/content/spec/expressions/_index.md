---
title: Expressions
layout: spec
weight: 7
---

# Expressions

### Syntax

{% apply grammkit %}
expression
  = ("(" expression ")")
  / destructuring_expression
  / typecheck_expression
  / pipe_expression
  / throw_expression
  / binary_operation_expression
  / unary_binary_expression
  / access_expression
  / expression_term

destructuring_expression = pattern ":=" expression

typecheck_expression = expression "is" "not"? type_expression

pipe_expression = expression "|>" function_call_expression

throw_expression = "throw" expression

binary_operation_expression
  = expression (
    logic_binary_operator
    / bitwise_arithmetic_binary_operator
    / inclusion_binary_operator
    / comparison_binary_operator
    / bitwise_shift_binary_operator
    / concatenation_binary_operator
    / arithmetic_binary_operator
  ) expression

unary_operation_expression
  = (
    arithmetic_unary_operator
    / logic_unary_operator
    / bitwise_arithmetic_unary_operator
  ) expression

access_expression = expression "." identifier

expression_term
  = literal_expression
  / tuple_expression
  / namedtuple_expression
  / list_expression
  / list_headtail_expression
  / let_expression
  / match_expression
  / cond_expression
  / receive_expression
  / tailrec_expression
  / function_call_expression
  / effect_call_expression
  / spawn_expression
  / do_expression
  / variable

literal_expression = atom / bool / number / string

tailrec_expression
  = ("recurse" "[" expression ("," expression)* ","? "]")
  / ("final" "[" expression "]")

variable = identifier
{% endapply %}
