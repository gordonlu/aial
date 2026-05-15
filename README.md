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
| `aial-rt/` | Runtime: C ABI static library (SQLite, HTTP, JSON, I/O, crossterm) |
| `aial-vscode/` | VS Code extension: syntax highlighting |
| `selfhost/` | **Self-hosting compiler**: AIAL compiler written in AIAL |
| `docs/` | Language specification & grammar |

## Features

- **`ask` keyword** — first-class AI invocation with model selection, streaming, context management, tool calls
- **Generics** — `fn id<T>(x: T) -> T`, `struct Container<T> { value: T }` with IR monomorphization
- **Module system** — `module Name { fn ... }` for code organization, nested modules
- **Actor model** — `actor::spawn/send/recv/try_recv/recv_timeout` with threaded `spawn_handler`
- **Tool calls** — `#[tool]` attribute, SSE tool_calls parsing, multi-turn tool loops
- **Thinking mode** — DeepSeek-V4 reasoning_content with gray dimmed display
- **`defer` statement** — LIFO cleanup blocks at function exit
- **Match exhaustiveness** — compiler enforces all enum variants covered
- **LLVM AOT** — native binary compilation via clang linkage
- **Self-hosting** — `selfhost/compiler.aal` compiles AAL source to LLVM IR → clang → binary

## Standard Library (80+ functions)

HTTP, JSON, SQLite memory, Map, Heap, Array (with sort), I/O, HTML escape, time, FFI, token estimation, process::run, args, int_to_string, string_to_int, str_find, multi-line input.

See [Guide for AI.md](Guide%20for%20AI.md) for full reference.

## Self-Hosting

```bash
cd selfhost
aial build compiler.aal
clang aial_output.ll -L ../aial-rt/target/release -laial_rt -lm -lpthread -ldl -rdynamic -o aialc
./aialc hello.aal       # Self-hosted compiler compiles hello.aal
```

## License

Apache 2.0
