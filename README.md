# rust-template

[![Checks](https://github.com/inc4/rust-template/workflows/Checks/badge.svg)](https://github.com/inc4/rust-template/actions?query=workflow%3A%22Checks%22)

This template can be used when creating a new Rust repository. Choose it when
you create a repository in the GitHub web interface, or use `--template` flag
with `gh repo create`.

## Getting started

- Review all files if you haven't done so before.
- Remove examples and rewrite this README.
- Change the name `rust-template` everywhere: `Cargo.toml`, `Dockerfile`,
  workflow files in `.github/workflows/`.

## Layout

- `src/main.rs` — the binary entry point. Keep it thin: argument parsing,
  wiring, startup. Most logic belongs in the library.
- `src/lib.rs` — the library root. Add modules here. Anything reusable or
  testable in isolation goes into the library, not the binary.
- Unit tests are the primary form of testing. They live alongside the code
  they cover, in a `#[cfg(test)] mod tests` block at the bottom of each
  source file.
- `tests/` — integration tests. Optional: not every project needs them.

## Build and run

```sh
cargo run -- --listen 0.0.0.0:8080
curl localhost:8080/hello
```

## Check, lint, test

```sh
cargo fmt --all -- --check
cargo clippy --tests -- -D warnings
cargo test
```

## Docker

```sh
docker build -t rust-template .
docker run --rm -p 8080:8080 rust-template
```
