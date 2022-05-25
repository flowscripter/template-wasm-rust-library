# template-wasm-rust-library

[![version](https://img.shields.io/github/v/release/flowscripter/template-wasm-rust-library?sort=semver)](https://github.com/flowscripter/template-wasm-rust-library/releases)
[![build](https://img.shields.io/github/workflow/status/flowscripter/template-wasm-rust-library/release-wasm-rust-library)](https://github.com/flowscripter/template-wasm-rust-library/actions/workflows/release-wasm-rust-library.yml)
[![dependencies](https://deps.rs/repo/github/flowscripter/template-wasm-rust-library/status.svg)](https://deps.rs/crate/flowscripter_template_wasm_rust_library)
[![rust doc](https://img.shields.io/docsrs/flowscripter_template_wasm_rust_library)](https://docs.rs/flowscripter_template_wasm_rust_library)
[![license: MIT](https://img.shields.io/github/license/flowscripter/template-wasm-rust-library)](https://github.com/flowscripter/template-wasm-rust-library/blob/main/LICENSE)

> Project template for a Rust library compiled to WASM.

## Usage

1. Use as a
   [template](https://docs.github.com/en/github/creating-cloning-and-archiving-repositories/creating-a-repository-from-a-template)
   to create a new repository.
2. Update links and references in `README.md`.

## Development

Install [wasm-pack](https://rustwasm.github.io/wasm-pack/):
`cargo install wasm-pack`

Install [Firefox](https://www.mozilla.org/firefox/browsers)

Build: `wasm-pack build --target web`

Test: `wasm-pack test --headless --firefox`

Lint: `cargo fmt`

## Documentation

### Overview

PNG image generated from `images/uml_diagram.mermaid`:

![UML Diagram](images/uml_diagram.png "UML Diagram")

### API

Link to auto-generated API docs for the library:

[API Documentation](https://docs.rs/flowscripter_template_wasm_rust_library)

## License

MIT © Flowscripter
