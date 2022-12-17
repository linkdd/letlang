---
title: Operators
weight: 4
---

# Expression syntax

```bnf
<proposition> :=
  | <proposition-let>
  | <proposition-expression>
  ;

<proposition-expression> :=
  <expression>
  ;

<expression> :=
  | "(" <expression> ")"
  | <expression-term>
  | <expression-binop>
  | <expression-unop>
  | <expression-pattern-match>
  | <expression-typecheck>
  | <expression-generic-specialization>
  | <expression-function-call>
  | ...
  ;

<expression-term> :=
  | <expression-symbol>
  | <value-literal>
  | <tuple-literal>
  | <struct-literal>
  | <list-literal>
  | <match-block>
  | <cond-block>
  | ...
  ;

<expression-binop> :=
  <expression> <binary-operator> <expression>
  ;

<expression-unop> :=
  <unary-operator> <expression>
  ;

<expression-pattern-match> :=
  <pattern> ":=" <expression>
  ;

<expression-typecheck> :=
  | <expression> "is" <typeref>
  | <expression> "is not" <typeref>
  ;

<expression-generic-specialization> :=
  <expression> "<" <typeref> ("," <typeref>)* ">"
  ;

<expression-function-call> :=
  <expression> "(" <expression> ("," <expression>)* ")"
  ;

<expression-symbol> :=
  <identifier> ("::" <identifier>)*
  ;

<value-literal> :=
  <literal>
  ;

<tuple-literal> :=
  "(" <expression> ("," <expression>)* ")"
  ;

<struct-literal> :=
  "{" <struct-literal-member> ("," <struct-literal-member>)* "}"
  ;

<struct-literal-member> :=
  <identifier> ":" <expression>
  ;

<list-literal> :=
  "[" <expression> ("," <expression>)* "]"
  ;
```

# Operator Precedence and associativity

Operators with the lowest precedence **MUST** be evaluated before operators with
a higher precedence.

Expressions within parentheses **MUST** be evaluated first.

{{< operator-table >}}
