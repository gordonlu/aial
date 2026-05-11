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
clang aial_output.ll -L ../aial-rt/target/release -laial_rt -o aial_hello
./aial_hello

# Add an API key
cargo run -- key add --provider deepseek --key sk-xxx
```

---

## Standard Library (24 functions)

```
HTTP (11)  get | post | post_json | header_map | header_set | status | text
           start | listen | respond | body
JSON (9)   parse | stringify | get | get_or | to_string | to_int | to_float
           array_len | array_get
IO (2)     readln | readln_timeout
AI (1)     ask::read_token (streaming)
HTML (1)   html::escape
```

---

## Showcase

### Web API Server (16 lines)

```aial
fn main() {
    let server = http::start(8080);
    loop {
        let req = http::listen(server);
        let body = http::body(req);
        let safe = html::escape(body);
        let resp = strcat("{\"echo\":\"", safe);
        resp = strcat(resp, "\"}");
        http::respond(req, resp, "application/json");
    }
}
```

```bash
$ curl -X POST -d '<script>alert(1)</script>' http://localhost:8080/
{"echo":"&lt;script&gt;alert(1)&lt;/script&gt;"}
```

### Streaming AI Chat (TUI)

```aial
fn main() {
    loop {
        print("You: ");
        let input = io::readln();
        if input == "exit" { break; }
        let stream = ask(model=0, prompt=input, stream=true, max_tokens=256);
        print("AI: ");
        loop {
            let token = ask::read_token(stream);
            if token == "" { break; }
            print(token);
        }
        println("");
    }
}
```

### JSON Never Crashes

```aial
let val = json::parse(config_text);
match json::type_of(val) {
    -1 => println("parse failed — fallback to defaults"),
    _ => {
        let port = json::get_or(json::get(val, "server"), "port", json::parse("8080"));
        let host = json::get_or(json::get(val, "server"), "host", json::parse("\"0.0.0.0\""));
    }
}
```

### Self-Hosting Lexer

```aial
// aial_lexer.aal — AIAL lexer written in AIAL, compiled to native via LLVM
fn main() {
    let src = file::read("examples/01_hello.aal");
    let n = strlen(src);
    let pos = 0;
    while pos < n {
        // ... token scanning ...
    }
}
```

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

---

## CLI

```bash
aial run <file.aal>                           # Interpreter
aial build <file.aal>                         # LLVM AOT → aial_output.ll
aial check <file.aal>                         # Syntax + type check
aial key add --provider <name> --key <key>    # Store API key
aial key list                                 # List keys (masked)
aial key remove --provider <name>             # Remove key
```

---

## VS Code Extension

```bash
ln -s $(pwd)/../aial-vscode ~/.vscode/extensions/aial-lang.aial
```

Features: syntax highlighting, bracket matching, comment toggling, module-aware coloring (http, json, context, file).

---

## Examples

| File | What it shows |
|------|--------------|
| `01_hello.aal` | Basic ask + println |
| `02_parallel.aal` | ask.race for parallel AI calls |
| `03_budget.aal` | Context token budget enforcement |
| `04_loop.aal` | For loop + conditional ask |
| `05_match.aal` | Match exhaustiveness |
| `06_webui.aal` | Built-in HTTP server |
| `07_json.aal` | JSON parse/get/stringify |
| `08_stream.aal` | Streaming AI + TUI chat |
| `aial_lexer.aal` | Self-hosting lexer |

---

## License

Apache 2.0 — see [LICENSE](LICENSE).
