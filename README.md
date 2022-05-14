# Letlang

![status: WIP](https://img.shields.io/badge/status-WIP-red)
![version: 0.0.0](https://img.shields.io/badge/version-v0.0.0-brightgreen)
![license: MIT](https://img.shields.io/badge/license-MIT-blue)

## Contributing

If you want to contribute, please make a pull request.

**Requirements:**

 - Python 3.10+
 - Rust & Cargo 1.59+
 - Hugo ^0.85 (we use a deprecated markdown renderer)

**Setup the environment:**

```
$ poetry install
$ poetry shell
(.venv)$
```

**Build:**

```
(.venv)$ poe build
```

**Run with the example:**

```
(.venv)$ poe run -p ./examples/hello-world/letproject.toml build
(.venv)$ cd ./examples/hello-world/.llbuild/exe/hello
(.venv)$ cargo build
```

> **NB:**
>  - The cargo step will be integrated to the compiler later.
>  - Currently, the generated crates reference the runtime in this source tree,
>    later it will be replaced by a published version on https://crates.io

## License

**Letlang**'s source code is distributed under the terms of the
[MIT License](./LICENSE.txt)

The website's content is distributed under the terms of the
[CC BY NC SA 4.0 License](./www/LICENSE.txt).
