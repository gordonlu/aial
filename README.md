# AIAL (枢言)

AIAL is a programming language designed for AI agents, not humans.

The only language where `ask` is a first-class keyword — no imports, no JSON wrangling, no retry logic.

```aial
fn main() {
    let answer = ask(model = 0, prompt = "What is the meaning of life?");
    println(answer.text);
}
```

## What Problem This Solves

AI application development today requires stitching together SDKs, handling API keys, managing context windows, parsing SSE streams, sanitizing output, and writing retry logic. AIAL absorbs all of this into the language runtime. `ask` is a keyword. `context::new(token_budget=N)` enforces cost control. `match` handles AI response variants. `api_key` type prevents accidental logging.

## Quick Start

```bash
git clone https://github.com/gordonlu/aial.git
cd aial
bash build.sh
aial run examples/01_hello.aal       # interpreter
aial build examples/01_hello.aal     # LLVM AOT → native binary
```

Prerequisites: Rust, clang (`apt install build-essential clang`)

## Architecture

```
Source (.aal) → Lexer → Parser → Name Resolver → Type Checker → IR Builder → IR Lower → LLVM Backend → clang → native binary
                                                                              ↘ Interpreter (dev mode)
```

## Features

- `ask` keyword — model selection, streaming, context, tool calls, thinking mode
- Generics with IR monomorphization — `fn id<T>(x: T) -> T`
- Actor concurrency — `actor::spawn/send/recv`
- Opaque types — `api_key` type rejected by print/serialize at compile time
- Defer — LIFO cleanup blocks
- Module system — `module Name { fn ... }`
- Self-hosting — AIAL compiler written in AIAL (`selfhost/compiler.aal`)

## Runtime Capabilities (80+ built-in functions)

| Category | Functions |
|----------|-----------|
| AI | `ask`, `ask::read_token`, `context::new`, `context::add_message` |
| HTTP | `http::get`, `http::post`, `http::start`, `http::listen`, `http::respond` |
| JSON | `json::parse`, `json::get`, `json::stringify`, `json::to_int` |
| Terminal | `term::clear`, `term::height`, `term::redraw`, `term::display_width` |
| I/O | `io::readkey`, `io::readln`, `io::raw_mode`, `io::is_tty` |
| String | `strlen`, `strcat`, `strslice`, `strchr`, `str_find`, `str_prev_char`, `str_next_char` |
| Map | `map::new`, `map::set`, `map::get`, `map::has` |
| Array | `array::new`, `array::push`, `array::get`, `array::sort`, `array::join` |
| Heap | `heap::new`, `heap::push`, `heap::pop`, `heap::peek` |
| File | `file::read`, `file::write`, `file::append`, `file::list_dir` |
| Process | `process::run`, `process::run_status` |
| Global | `global::set`, `global::get`, `global::has` |
| Key | `key::set`, `key::exists`, `key::delete` |
| Time | `time::now`, `time::now_ms`, `time::sleep` |
| FFI | `ffi::load`, `ffi::call`, `ffi::close` |
| Memory | `ctx::open_memory`, `ctx::save_message`, `ctx::load_messages` (SQLite-backed) |
| Convert | `int_to_string`, `string_to_int`, `token_estimate`, `html::escape` |

Full API reference: [Guide for AI.md](Guide%20for%20AI.md)

## How This Is Different

| Library / Framework | AIAL |
|---------------------|------|
| LangChain / LlamaIndex | Python libraries — glue code, JSON wrangling, retry logic in user code |
| OpenAI SDK | Language-agnostic HTTP wrapper — still requires manual streaming, error handling |
| Rust `llm` crates | IDE-dependent, no compiler enforcement of API key safety, cost budgets |

AIAL moves AI invocation from library calls to language semantics. The compiler checks API keys aren't leaked. The runtime enforces token budgets. `ask` is parsed, type-checked, and compiled like `if` or `for`.

## Testing

```bash
cargo test                           # 87 tests (44 unit + 43 integration)
```

## Self-Hosting

```bash
cd selfhost
aial build compiler.aal
clang aial_output.ll -L ../aial-rt/target/release -laial_rt -lm -lpthread -ldl -rdynamic -o aialc
./aialc hello.aal
```

## License

Apache 2.0
