---
title: Unix cat clone
---

```letlang
import stdlib as std
import argparse

class options(cfg: {
  verbose: boolean,
  file: string
})

class verbose_mode(cfg: options & { verbose: true })

effect log(level: "debug" | "info", message: string) -> :ok

func main(args: std.list<string>) -> std.result<()> {
  parser := argparse.parser()
  :ok = argparse.add_option(parser, {
    short: "v",
    long: "verbose",
    type: argparse.BOOL,
    default: false
  })
  :ok = argparse.add_option(parser, {
    short: "f",
    long: "file",
    type: argparse.STRING,
    required: true
  })

  match argparse.parse_args(parser, args) {
    (:ok, options(opts)) => {
      do {
        cat_file(opts.file)
      }
      effect log("info", message) {
        std.print(message)
      }
      effect log("debug", message) {
        if opts is verbose_mode {
          std.print("DEBUG:", message)
        }
        else {
          :ok
        }
      }
    }

    err => err
  }
}
```