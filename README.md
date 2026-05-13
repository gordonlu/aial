# AIAL (枢言)

AI-native programming language where `ask` is a first-class keyword.

```aial
fn main() {
    let answer = ask(model = 0, prompt = "What is the meaning of life?");
    println(answer.text);
}
```

## Quick Start

```bash
git clone https://github.com/gordonlu/aial.git
cd aial
bash build.sh                         # builds compiler + runtime
aial run examples/01_hello.aal       # interpreter (development)
aial build examples/01_hello.aal     # LLVM AOT (production)
```

**Prerequisites:** Rust, clang, libc-dev (`apt install build-essential clang`)

## Structure

| Directory | Purpose |
|-----------|---------|
| `aial-compiler/` | Compiler: lexer → parser → type checker → IR → LLVM backend |
| `aial-rt/` | Runtime: C ABI static library (SQLite, HTTP, JSON, I/O) |
| `aial-vscode/` | VS Code extension: syntax highlighting |
| `docs/` | Language specification & grammar |
| `deep-tui/` | *separate repo* — terminal AI chat built entirely in AIAL |

## Features

- **`ask` keyword** — first-class AI invocation with model selection, streaming, context management
- **Generics** — `fn id<T>(x: T) -> T`, `struct Container<T> { value: T }` with monomorphization
- **Module system** — `module Name { fn ... }` for code organization, nested modules
- **Actor model** — `actor::spawn/send/recv/try_recv/recv_timeout` with threaded `spawn_handler`
- **`defer` statement** — LIFO cleanup blocks at function exit
- **Match exhaustiveness** — compiler enforces all enum variants covered
- **LLVM AOT** — native binary compilation via clang linkage

## Standard Library (50+ functions)

HTTP, JSON, SQLite memory, Map, Heap, Array (with sort), I/O, HTML escape, time, FFI, token estimation. See [Guide for AI.md](Guide%20for%20AI.md).

## Projects

- **[Deep TUI](https://github.com/gordonlu/deep-tui)** — terminal AI chat with streaming, shortcuts, bracketed paste

## License

Apache 2.0
