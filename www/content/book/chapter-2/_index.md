---
title: 2. Pattern Matching
description: Description of Letlang's pattern matching mechanism
prev: /book/chapter-1/
next: /book/chapter-2/1-match-operator
---

# Introduction

Variables are bound to values through the **pattern matching** mechanism.

Pattern matching occurs when evaluating a `match` expression, a `receive` block,
a `do`-`catch`-`intercept` expression and a match operator (`:=`) expression.

In a pattern matching, a left-hand side pattern is matched against a right-hand
side expression. If the matching succeeds, any unbound variables in the pattern
become bound. If the matching fails, a run-time error occurs.
