# AIAL (枢言)

AI-native programming language where `ask` is a first-class keyword.

```aial
fn main() {
    let answer = ask(model = 0, prompt = "What is the meaning of life?");
    println(answer.text);
}
```

## Structure

| Directory | Purpose |
|-----------|---------|
| `aial-compiler/` | Compiler — lexer, parser, type checker, IR, backends |
| `aial-rt/` | Runtime — C ABI static library for AOT linkage |
| `aial-vscode/` | VS Code extension — syntax highlighting |
| `docs/` | Language specification, grammar, IR docs |

## Quick Start

```bash
cd aial-compiler
cargo build --release
cargo run -- run examples/01_hello.aal
```

See [aial-compiler/README.md](aial-compiler/README.md) for full documentation.
