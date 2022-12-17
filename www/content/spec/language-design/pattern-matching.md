---
title: Pattern Matching
weight: 3
---

# Pattern syntax

A pattern can be either:

 - a variable binding: `a`
 - an expression: `$(1 + 2)`
 - a value literal: `42`, or `"hello"`
 - a tuple whose elements are patterns: `(@ok, result)` or `(4, "hello")`
 - a structure whose values are patterns: `{foo: bar}` or `{foo: 42}`
 - a list whose elements are patterns: `[@ok, result]` or `[4, "hello"]`
 - a list destructuring where we match the pattern and tail of the list: `[head | tail]`

```bnf
<pattern> :=
  | <pattern-variable-binding>
  | <pattern-expression>
  | <pattern-value-literal>
  | <pattern-tuple-literal>
  | <pattern-struct-literal>
  | <pattern-list-literal>
  | <pattern-list-destructuring>
  ;

<pattern-variable-binding> :=
  <identifier>
  ;

<pattern-expression> :=
  "$(" <expression> ")"
  ;

<pattern-value-literal> :=
  <literal>
  ;

<pattern-tuple-literal> :=
  "(" <pattern> ("," <pattern>)* ")"
  ;

<pattern-struct-literal> :=
  "{" <pattern-struct-literal-member> ("," <pattern-struct-literal-member>)* "}"
  ;

<pattern-struct-literal-member> :=
  <identifier> ":" <pattern>
  ;

<pattern-list-literal> :=
  "[" <pattern> ("," <pattern>)* "]"
  ;

<pattern-list-destructuring> :=
  "[" <pattern> "|" <pattern> "]"
  ;

<literal> :=
  | <literal-bool>
  | <literal-number>
  | <literal-string>
  | <literal-atom>
  ;

<literal-bool> :=
  | "true"
  | "false"
  ;

<literal-number> :=
  | /0b_*[01][_01]*/
  | /0o_*[0-7][_0-7]*/
  | /[1-9][_1-9]*/
  | /0x_*[0-9a-fA-F][_0-9a-fA-F]*/
  | /((\d+\.?\d*)|(\.\d+))(([eE][+-]?)?\d+)?/
  ;

<literal-string> :=
  /"(?:[^"]|\\")*"/
  ;

<literal-atom> :=
  /@(('(?:[^']|\\')+')|([_a-zA-Z][_a-zA-Z0-9]*))/
  ;

<identifier> :=
  /[_a-zA-Z][_0-9a-zA-Z]*/
  ;
```

# Pattern-match expressions

A pattern-match expressions uses the operator `:=` and tries to match the
pattern on the left to the expression on the right.

Syntax:

```bnf
<pattern-match-expression> :=
  <pattern> ":=" <expression>
  ;
```

Examples:

```letlang
a := 1;  # a is now bound to the value 1
a := 2;  # a is now bound to the value 2

(a, b) := (1, 2);  # a is bound to 1, and b is bound to 2

{x: a, y: b} := {x: 1, y: 1};  # a is bound to 1, and b is bound to 2

[a, b] := [1, 2];  # a is bound to 1, and b is bound to 2

[head | tail] := [1, 2, 3];  # head is bound to 1, and tail is bound to [2, 3]

(@ok, val) := (@ok, 42);   # val is bound to 42
```

A pattern-match expression **MUST** return the matched value:

```letlang
(a, 2) := (1, b) := (1, 2);  # a is bound to 1, and b is bound to 2
```

If a pattern-match expression can't be matched, the Letlang runtime **MUST** throw
an exception of the form:

> `(@match_error, "<stringified value>")`

# Pattern-match branching

A `match{}` code-block will try to match the supplied expression to one of the
branches pattern.

The `match{}` code-block **MUST** execute the first branch's code-block that
match and evaluates to its return value.

If no branch matches, the Letlang runtime **MUST** throw an exception of the
form:

> `(@match_error, "<stringified value>")`

Syntax:

```bnf
<match-block> :=
  "match" <expression> "{"
    <match-block-branch> ("," <match-block-branch>)*
  "}"
  ;

<match-block-branch> :=
  | <pattern> "=>" "{" <proposition>+ "}"
  | <pattern> "=>" <expression>
  ;
```

Example:

```letlang
is_ok := match result {
  (@ok, value) => true,
  (@error, reason) => false,
};
```

# Conditional branching

A `cond{}` code-block **MUST** execute the first branch's code-block whose
condition evaluates to `true`.

If no branch evaluates to `true`, then the `else` default branch **MUST** be
executed.

The `cond{}` code-block **MUST** evaluate to the value returned by the executed
branch.

Syntax:

```bnf
<cond-block> :=
  "cond" "{"
    <cond-block-branch> ("," <cond-block-branch>)*
    <cond-block-else-branch>
  "}"
  ;

<cond-block-branch> :=
  | <expression> "=>" "{" <proposition>+ "}"
  | <expression> "=>" <expression>
  ;

<cond-block-else-branch> :=
  | "else" "=>" "{" <proposition>+ "}"
  | "else" "=>" <expression>
  ;
```

Example:

```letlang
can_edit := cond {
  role = "admin" => true,
  role = "moderator" => true,
  else => false,
};
```

# Let propositions

A `let` proposition defines a constraint on the value of a variable binding.

If the variable is not yet bound, the constraint **MUST** be enforced as soon as
it is bound.

If the variable is already bound to a value, the constraint **MUST** be enforced
immediately.

Every time a constrained variable is bound to a new value, the constraint
**MUST** be enforced.

If a constraint fails to be enforced (the value is not valid), the runtime
**MUST** throw an exception of the form:

> `(@type_error, (@constraint, "<variable name>"))`

A `let` proposition always specify a type for the variable, and optionnaly one
or more expressions that all must evaluate to `true` for the variable.

Syntax:

```bnf
<proposition> :=
  | <proposition-let>
  | <proposition-expression>
  ;

<proposition-expression> :=
  <expression>
  ;

<proposition-let> :=
  "let" <identifier> ":" <typeref> <proposition-let-checks>? ";"
  ;

<proposition-let-checks> :=
  "{" <expression> ("," <expression>)* "}"
  ;
```

Examples:

```letlang
let a: int;

a := 1;    # OK
a := 2.3;  # TYPE ERROR
```

```letlang
let a: number;
let b: number { b > a };

a := 1;
b := 5;  # OK
b := 0;  # TYPE ERROR
```
