---
title: 4. Concurrency
description: Description of Letlang's concurrency model
prev: /book/chapter-3/
next: /book/chapter-4/1-functions
---

# Introduction

**Letlang**'s concurrency model is heavily inspired by
[Erlang](https://www.erlang.org/) and [Elixir](https://elixir-lang.org/).
It is implemented using the Rust crate [tokio](https://tokio.rs), and is
**single-threaded**.

{{< center >}}
![Concurrency VS Parallelism Diagram](/img/letlang-concurrency.svg)
{{< /center >}}
