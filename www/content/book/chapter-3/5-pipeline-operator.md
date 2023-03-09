---
title: 3.5. Pipeline Operator
description: Chaining computations in Letlang
prev: /book/chapter-3/4-exceptions
next: /book/chapter-4/
---

Chaining computations can sometimes be awkward.

When you have quite a number of steps, each requiring the output of the previous
step, the code can become hard to read.

{{< columns >}}
{{< column class="is-half" >}}
{{< markdown >}}
```python
list(map(
  lambda x: x * 2,
  filter(
    lambda x: x % 2 == 0,
    range(10, 20)
  )
))
```
{{< /markdown >}}
{{< /column >}}

{{< column class="is-half" >}}
{{< markdown >}}
This Python code will produce the following list:

```
[20, 24, 28, 32, 36]
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

[Elixir](https://elixir-lang.org/), [F#](https://fsharp.org/),
[R](https://www.r-project.org/) (and probably many other languages) all have a
solution for this: **the pipeline operator**.

A pipeline will transform some data by applying sequentially computations, each
depending on the result of the previous one, and producing an output that will
be forwarded to the next.

If python had a pipeline operator (`|>` for example), the previous code could
be rewritten as:

{{< columns class="is-centered" >}}
{{< column class="is-half" >}}
{{< markdown >}}
```python
range(10, 20)
  |> filter(lambda x: x % 2 == 0)
  |> map(lambda x: x * 2)
  |> list()
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}

Which is definitely clearer.

Here is how it works in the following languages:

| Language | Operator Syntax | Description |
| --- | --- | --- |
| Elixir | `val \|> func()` | Injects the lefthand-side value as first argument of the righthand-side function call |
| F# | `val \|> func` or `func <\| val` | Injects the value on one side to the function call on the opposite side |
| R | `val %>% func` | Injects the lefthand-side value as first argument of the righthand-side function call |

In **Letlang**, the pipeline operator mimics the one from Elixir:

{{< columns class="is-centered" >}}
{{< column class="is-half" >}}
{{< markdown >}}
```letlang
x |> add(5) |> mul(2)
# equivalent to
mul(add(x, 5), 2)
```
{{< /markdown >}}
{{< /column >}}
{{< /columns >}}
