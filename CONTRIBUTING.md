# Contributing to AIAL

Thanks for your interest! AIAL is early-stage and every contribution counts.

## Quick Start

```bash
git clone https://github.com/gordonlu/aial
cd aial
bash build.sh
```

## Development

### Building

```bash
cd aial-compiler
cargo build --release
```

### Running Tests

```bash
cargo test                           # all tests
cargo test --test integration        # integration tests only
```

Tests use `AIAL_MOCK=1` by default — no API key needed.

### Project Structure

| Directory | Purpose |
|-----------|---------|
| `aial-compiler/` | Rust compiler (lexer → parser → type checker → IR → LLVM) |
| `aial-rt/` | Rust runtime library (C ABI, linked into AOT binaries) |
| `aial-vscode/` | VS Code syntax highlighting extension |
| `selfhost/` | Self-hosting compiler written in AIAL |
| `docs/` | Language spec, type system, stdlib reference |

### Code Conventions

- Keep comments in English
- Follow Rust 2021 idioms
- No unsafe code unless absolutely necessary (and documented)
- Match existing error reporting style via `philosophy.rs`

## Making Changes

1. Open an issue to discuss non-trivial changes first
2. Fork and create a feature branch
3. Run `cargo test` before submitting
4. Submit a PR against the `main` branch

## License

Apache 2.0 — see [LICENSE](LICENSE).
