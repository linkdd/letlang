---
title: Print file to stdout
description: Demonstration of effects and argparse module.
---

```letlang
module "cat.main";

import "std";
import "argparse";

class options(cfg: {
  verbose: boolean,
  file: string
});

class verbose_mode(cfg: options & { verbose: true });

effect log(level: "debug" | "info", message: string) -> @ok;

func main(args: std.list<string>) -> std.result<()> {
  parser := argparse.parser()
  let @ok = argparse.add_option(parser, {
    short: "v",
    long: "verbose",
    type: argparse.BOOL,
    default: false
  });
  let @ok = argparse.add_option(parser, {
    short: "f",
    long: "file",
    type: argparse.STRING,
    required: true
  });

  match argparse.parse_args(parser, args) {
    (@ok, options(opts)) => {
      do {
        cat_file(opts.file);
      }
      effect log("info", message) {
        std.print(message);
      }
      effect log("debug", message) {
        if opts is verbose_mode {
          std.print("DEBUG:", message);
        }
        else {
          @ok;
        };
      };

      (@ok, ());
    },
    err => err
  };
}
```