---
title: Functions
weight: 5
---

# Function Syntax

```bnf
<function-statement> :=
  [ "pub" ] "func" <identifier>
  [ <function-type-parameters> ]
  "(" <function-call-parameters> ")" "->" <type-ref>
  "{" <proposition>+ "}"
  ;

<function-type-parameters> :=
  "<" <identifier> ("," <identifier>)* ">"
  ;

<function-call-parameters> :=
  <function-call-parameter> ("," <function-call-parameter>)*
  ;

<function-call-parameter> :=
  <identifier> ":" <type-ref>
  ;
```

The value the last proposition of the function's body evaluates to is the return
value of the function.

Examples:

```letlang
pub func add(a: number, b: number) -> number {
  a + b;
}
```

# Generic Functions

Type parameters of functions **MUST** be resolved when calling the function.

If not specified, type parameters **MAY** be inferred from the function call
parameters.

Example:

```letlang
func swap<T, U>(a: T, b: U) -> (U, T) {
  (b, a);
}
```

```letlang
# explicit type parameters:
(42, "hello") := swap<string, int>("hello", 42);

# inferred type parameters:
(42, "hello") := swap("hello", 42);
```

# Tail Recursion Elimination

Tail calls in functions **MAY** be eliminated.

Letlang provides an explicit syntax for tail-recursive functions:

```bnf
<tailrec-function-statement> :=
  [ "pub" ] "tailrec" <identifier>
  [ <function-type-parameters> ]
  "(" <function-call-parameters> ")" "->" <type-ref>
  "{" <proposition>+ "}"
  ;

<expression-term> :=
  | ...
  | <tailrec-final>
  | <tailrec-recurse>
  | ...
  ;

<tailrec-final> :=
  "final" "[" <expression> "]"
  ;

<tailrec-recurse> :=
  "recurse" "[" <expression> ("," <expression>)* "]"
  ;
```

Tail calls in a tail-recursive function **MUST** be eliminated.

A tail-recursive function **MUST** return either a `final[...]` value or a
`recurse[...]` value:

 - `final[...]` values **MUST** terminate the function's execution and return the wrapped value
 - `recurse[...]` values **MUST** continue the function's execution with the wrapped values as arguments for the new iteration

Example:

```letlang
tailrec fizzbuzz_impl(n: int, limit: int) -> @ok {
  cond {
    n < limit => {
      cond {
        n % 3 = 0 and n % 5 = 0 => {
          std::io::println("fizz buzz");
        },
        n % 3 = 0 => {
          std::io::println("fizz");
        },
        n % 5 = 0 => {
          std::io::println("buzz");
        },
        else => {
          std::io::println("" <> n);
        },
      };

      recurse[n + 1, limit];
    },
    else => {
      final[@ok];
    }
  };
}

func fizzbuzz(limit: int) -> @ok {
  fizzbuzz_impl(0, limit);
}
```
