---
title: Tokens
layout: spec
weight: 6
---

# Tokens

Tokens are primitive productions in the grammar defined by regular (non-recursive)
languages. Letlang source input can be broken down into the following kinds of
tokens:

 - [Keywords](./spec/lexical-structure/keywords)
 - [Identifiers](./spec/lexical-structure/identifiers)
 - [Literals](./spec/lexical-structure/tokens/#literals)
 - [Punctuation](./spec/lexical-structure/tokens/#punctuation)
 - [Operators](./spec/lexical-structure/tokens/#operators)

# Literals

### Syntax

{% apply grammkit %}
string = '"' ( [^"\\] / '\\' . )* '"'

atom = '@' ( ( "'" ([^'] / "\\'" )+ "'" ) / [_a-zA-Z0-9]+ )

bool = "true" / "false"

number
  = ( "0b" "_"* [01] [_01]* )
  / ( "0o" "_"* [0-7] [_0-7]* )
  / ( [0-9] [_0-9]* )
  / ( "0x" "_"* [0-9a-fA-F] [_0-9a-fA-F]* )
  / ( ( ( [0-9]+ "."? [0-9]* ) / ( "." [0-9]+ ) ) ( ( [eE] [+-]? )? [0-9]+ )? )
{% endapply %}


# Punctuation

| Token | Syntax |
| --- | --- |
| Module separator | `::` |
| Parenthesis | `(`, `)` |
| Brackets | `[`, `]` |
| Braces | `${`, `{`, `}` |
| Comma | `,` |
| Dot | `.` |
| Colon | `:` |
| Semicolon | `;` |
| Arrow | `->` |
| Pattern ignore | `_` |
| Ellipsis | `...` |
{.table-hoverable .table-col0-minw .table-col0-left .table-col1-left}


# Operators

| Token | Syntax |
| --- | --- |
| Match | `:=` |
| Arithmetic | `+`, `-`, `*`, `/`, `%`, `**` |
| Comparison | `<`, `<=`, `=`, `!=`, `>=`, `>` |
| Negation | `!`, `¬` |
| Bitwise | `~`, `&`, `|`, `^`, `<<`, `>>` |
| Logic | `not`, `and`, `or` |
| String | `<>` |
| Lists | `++`, `in` |
| Type | `is` |
| Pipeline | `|>` |
{.table-hoverable .table-col0-minw .table-col0-left .table-col1-left}
