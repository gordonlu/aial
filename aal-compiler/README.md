# AAL (灵枢) — The AI-Native Programming Language

> **"The best framework is the one you don't need."**  
> AAL is a programming language where AI calls are first-class citizens — not a library, not a framework, but a language built from the ground up for the age of intelligence.

---

```aal
fn main() {
    let ctx = context::new(token_budget = 4096);
    let answer = ask(model = 0, prompt = "What is the meaning of life?");
    println(answer.text);
}
```

```bash
$ AAL_MOCK=1 aal run
[AI Call] provider=openai, model=gpt-4o, prompt="What is the meaning of life?"
The meaning of life is a question that has been explored by philosophers...
```

No imports. No JSON wrangling. No retry logic. No token counting. **The language handles the complexity.**

---

## Why AAL?

Every AI application today faces the same problems — and solves them the same ad-hoc way:

| Problem | Everyone else | AAL |
|---------|--------------|-----|
| **API keys** | Env vars + hope they don't leak | Opaque `api_key` type, compiler-forbidden to print/serialize |
| **Cost control** | Manual `if budget: count++` | `context::new(token_budget = N)` — runtime enforces hard cap |
| **Error handling** | try/except everywhere | `ask.many` — parallel fallbacks, built-in retry |
| **Structured output** | "Return JSON" in prompt + parse | `ask<MyType>(...)` — type-safe, schema-enforced |
| **Observability** | Manually log each call | Every `ask` auto-instrumented by the runtime |

**AAL is not a library you import. It is a language that absorbs complexity so you don't have to.**

---

## Philosophy: Three Pillars

AAL's design is rooted in three classical Chinese principles:

### 少则得 — Less is More
> *"To attain knowledge, add things every day. To attain wisdom, remove things every day."* — Laozi

Only **20 keywords**. The standard library is minimal by design. A single keyword `ask` replaces hundreds of lines of API boilerplate. The compiler bears the complexity so your code stays readable.

### 法不可违 — The Law Cannot Be Broken
> *"The law is the codification of governance, not to be bent by personal whim."* — Han Feizi

Security and safety are enforced by the compiler and runtime together, never left to developer discretion. An `api_key` value **cannot** be printed, serialized, or leaked — the compiler rejects such code. A `token_budget` **will** stop further AI calls when exhausted — the runtime enforces it. No amount of developer fatigue can bypass these guarantees.

### 直指人心 — Directly Point to the Mind
> *"A special transmission outside the teachings, not dependent on words or letters."* — Chan Buddhism

The code says *what* you want, not *how* to get it. `ask` is an expression, not a ritual. `context` transparently manages session state. The language meets you at the level of intent, not implementation.

---

## Quick Start: 5 Minutes

### 1. Install

```bash
# Requires Rust 1.70+
git clone https://github.com/your-org/aal
cd aal/aal-compiler
cargo build --release
alias aal='./target/release/aal'
```

### 2. Run the demo

```bash
AAL_MOCK=1 aal run examples/hello_agent.aal
```

### 3. Add an API key

```bash
aal key add --provider openai --key sk-...
aal key list
# => openai: sk-t…xyz
```

### 4. Go live

```bash
aal run examples/hello_agent.aal
```

---

## Showcase: What Makes AAL Unique

### Budget enforcement (P0 — "Prevent the disease")

```aal
fn main() {
    // Runtime WILL stop after 1000 tokens — no if-checks needed
    let ctx = context::new(token_budget = 1000);

    loop {
        let response = ask(
            model = 0, context = ctx,
            prompt = "Tell me a joke",
            max_tokens = 500
        );
        println(response.text);
    }
    // After ~2 iterations, runtime throws:
    // "Token budget exhausted: 1000 used, 1000 budget"
}
```

### Parallel fallback (P1 — "Orthodox engages, surprise wins")

```aal
fn classify(text: string) -> string {
    let answers = ask.race([
        // Three models compete; first valid response wins
        (model = 0, prompt = "Classify: " + text, max_tokens = 50),
        (model = 1, prompt = "Classify: " + text, max_tokens = 50),
        (model = 2, prompt = "Classify: " + text, max_tokens = 50),
    ]);
    answers[0].text
}
```

### Structured output (P1 — "Straight to the heart")

```aal
// When format = 1, API returns JSON
let result = ask(
    model = 0,
    prompt = "Extract: name, email, age",
    format = 1
);
// result.text contains valid JSON the runtime parsed
```

---

## Architecture

```
Source Code (.aal)
    │
    ▼
┌─────────────┐
│   Lexer     │  → tokens
├─────────────┤
│   Parser    │  → AST (recursive descent + precedence climbing)
├─────────────┤
│ Name Resolv │  → Symbol table
├─────────────┤
│ Type Check  │  → Bidirectional type inference + capability audit
├─────────────┤
│  IR Builder │  → AAL-IR (SSA with AI-native instructions)
├─────────────┤
│  IR Lower   │  → Runtime calls (ExternCall)
├─────────────┤
│ Interpreter │  → Executes with budget enforcement + key injection
└─────────────┘
    │
    ▼
  Output
```

The entire pipeline is a single binary — no VM, no container, no cloud dependency. Just `cargo build` and you're done.

---

## CLI Reference

```bash
aal <file.aal>                          # Compile & run
aal run <file.aal>                      # Same, explicit
aal check <file.aal>                    # Syntax check only

aal key add --provider <name> --key <k> # Store API key (0600 perms)
aal key list                            # List keys (masked)
aal key remove --provider <name>        # Remove key

AAL_MOCK=1 aal run <file.aal>           # Mock mode (no API key needed)
AAL_KEY_OPENAI=sk-... aal run <file>    # Env-var key injection (CI)
```

---

## Project Status

AAL is in **early prototype** phase. The compiler pipeline is complete and functional, but many features are still minimal implementations:

| Feature | Status |
|---------|--------|
| Lexer + Parser | ✅ Complete |
| Type Checker | ✅ Basic (bidirectional inference) |
| IR + Lowering | ✅ Complete |
| Interpreter | ✅ Functional |
| `ask` keyword | ✅ Single, many, race |
| `context` management | ✅ Budget tracking |
| `api_key` safety | ✅ Opaque type + static checks |
| Capability system | ✅ `aal.toml` declaration + compile-time audit |
| `for` loops | ✅ |
| `if` expressions | ✅ |
| `match` + patterns | ⚠️ Basic (linear chain) |
| Structured output | ⚠️ `format = 1` (JSON mode) |
| Method calls | ⚠️ Flattened to function calls |
| JIT backend (Cranelift) | 🔄 Planned (interpreter for now) |
| Package manager | ❌ |
| LSP / IDE support | ❌ |
| Self-hosting | ❌ |

---

## Contributing

AAL is in its early days and every contribution matters. Areas that need love:

- **Language features**: `match` full patterns, struct literals, impl blocks
- **Runtime**: Real async streaming, tool registry, actor model
- **Backend**: Migrate from interpreter to Cranelift JIT
- **Ecosystem**: Package manager, LSP, standard library
- **Documentation**: Grammar reference, type system spec, tutorials

---

## License

Apache 2.0 — see [LICENSE](LICENSE).

---

*"The best leader is one whose existence is barely known by the people."* — Laozi
