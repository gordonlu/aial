# AIAL (枢言 Shuyan) — The AI-Native Programming Language

> *"To attain wisdom, remove things every day."* — Laozi

AIAL is a programming language where `ask` is a first-class keyword. No imports, no JSON wrangling, no retry logic — the language absorbs the complexity of AI application development.

```aial
fn main() {
    let answer = ask(model = 0, prompt = "What is the meaning of life?");
    println(answer.text);
}
```

## Why AIAL?

| Problem | You today | AIAL |
|---------|-----------|------|
| API keys | Env vars + pray they don't leak | `api_key` type — compiler rejects any print/serialize |
| Cost control | Manual budget bookkeeping | `context::new(token_budget = N)` enforced by runtime |
| 4xx/5xx panic | try-catch everywhere | HTTP status is just an int. `match` it |
| JSON parse crash | Config file typo → deploy fails | `json::parse` returns `JsonError`, never panics |
| XSS in agents | Manual santization | `html::escape` + opaque `ExternalText` type |
| Streaming | Async hell | `ask(stream=true)` + blocking `read_token` loop |
| Web server | Install Flask/FastAPI | `http::start` + `http::listen` — built in |
| AI error handling | Stringly-typed hope | `match` on `Success/Degraded/Refused/Error` |

---

## Quick Start

```bash
# Build the compiler
git clone https://github.com/gordonlu/aial
cd aial/aial-compiler
cargo build --release

# Build the runtime (needed for LLVM AOT)
cd ../aial-rt && cargo build --release && cd ../aial-compiler

# Run with interpreter (default)
cargo run -- run examples/01_hello.aal

# Compile to native binary (LLVM AOT)
cargo run -- build examples/01_hello.aal
clang aial_output.ll -L ../aial-rt/target/release -laial_rt -lm -lpthread -ldl -rdynamic -o aial_hello
./aial_hello

# Add an API key
cargo run -- key add --provider deepseek --key sk-xxx
```

---

## Standard Library (80+ functions)

| Category | Functions |
|----------|-----------|
| HTTP (11) | get, post, post_json, header_map, header_set, status, text, start, listen, respond, body, method, path |
| JSON (11) | parse, stringify, get, get_or, type_of, to_string, to_int, to_float, array_len, array_get |
| Map (5) | new, set, get, has, remove |
| Heap (5) | new, push, pop, peek, len |
| Array (5) | new, push, sort, get, len |
| IO (6) | readln, readln_timeout, readkey, readkey_timeout, raw_mode, read_multiline |
| Key (3) | set, exists, delete |
| File (5) | read, write, append, patch, list_dir |
| Context Memory (6) | open_memory, save_message, load_messages, load_messages_since, close_memory, last_error |
| Process (1) | run |
| FFI (3) | load, call, close |
| Term (6) | clear, height, setup, redraw, draw_text_clipped, cursor_row |
| Time (3) | sleep, now, now_ms |
| Line Editor (4) | new, read, redraw, end |
| String (7) | strlen, strcat, strslice, strchr, str_eq, starts_with, str_find |
| Token (1) | token_estimate |
| HTML (1) | escape |
| AI (1) | ask::read_token |
| Convert (2) | int_to_string, string_to_int |
| System (1) | args |

---

## Architecture

```
Source (.aal) → Lexer → Parser → Name Resolver → Type Checker
                                            ↓
              LLVM IR ← IR Lower ← IR Builder
                ↓
         clang + aial_rt.a → native binary

Backends: Interpreter (dev) | LLVM AOT (prod) | Cranelift JIT (planned)
```

## Features

- **`ask` keyword** — first-class AI invocation with model selection, streaming, context, tool calls, thinking mode
- **Generics** — `fn id<T>(x: T) -> T`, `struct Container<T> { value: T }` with IR monomorphization
- **Module system** — `module Name { fn ... }` with nested modules
- **Actor model** — `actor::spawn/send/recv/try_recv/recv_timeout/error` with threaded `spawn_handler`
- **Tool calls** — `#[tool]` attribute, SSE tool_calls parsing, multi-turn tool loops
- **Defer** — LIFO cleanup blocks at function exit
- **OpaqueStruct type tracking** — heap/array/map handles track element types at compile time
- **LLVM AOT** — native binary via clang, with proper f64 ABI, ptr↔int conversion
- **Self-hosting** — `selfhost/compiler.aal` compiles AAL to LLVM IR → clang → binary
- **crossterm input** — proper key event handling (arrow keys, CJK, paste)

## Testing

```bash
cargo test                           # 87 tests (44 unit + 43 integration)
cargo test --test integration        # Integration: compile→link→run
```

## CLI

```bash
aial run <file.aal>                           # Interpreter
aial build <file.aal>                         # LLVM AOT → aial_output.ll  
aial check <file.aal>                         # Syntax + type check
aial key add --provider <name> --key <key>    # Store API key
aial key list                                 # List keys (masked)
aial key remove --provider <name>             # Remove key
```

## VS Code Extension

```bash
ln -s $(pwd)/../aial-vscode ~/.vscode/extensions/aial-lang.aial
```

Features: syntax highlighting, bracket matching, comment toggling, module-aware coloring.

## Examples

| File | What it shows |
|------|--------------|
| `01_hello.aal` | Basic ask + println |
| `02_parallel.aal` | ask.race for parallel AI calls |
| `06_webui.aal` | Built-in HTTP server |
| `08_stream.aal` | Streaming AI + TUI chat |
| `selfhost/compiler.aal` | Self-hosting compiler |

## License

Apache 2.0 — see [LICENSE](LICENSE).
