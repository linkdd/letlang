# Letlang

![status: WIP](https://img.shields.io/badge/status-WIP-red)
![version: 0.0.0](https://img.shields.io/badge/version-v0.0.0-brightgreen)
![license: MIT](https://img.shields.io/badge/license-MIT-blue)

## Contributing

If you want to contribute, please make a pull request.

**Requirements:**

 - Rust & Cargo 1.59+
 - Hugo ^0.85 (we use a deprecated markdown renderer)


**Build:**

```
$ cargo build
```

**Run example:**

```
$ export LETLANG_RUNTIME_PATH="${PWD}/sources/letlang_runtime"
$ cargo run -- -p examples/dummy run
```

## License

**Letlang**'s source code is distributed under the terms of the
[MIT License](./LICENSE.txt)

The website's content is distributed under the terms of the
[CC BY NC SA 4.0 License](./www/LICENSE.txt).
