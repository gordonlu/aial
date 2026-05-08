# AAL (灵枢) — The AI-Native Programming Language

> *"To attain wisdom, remove things every day."* — Laozi

AAL is a programming language where AI calls are first-class citizens — not a library, not a framework, but a language built from the ground up for the age of intelligence.

```aal
fn main() {
    let ctx = context::new(token_budget = 4096);
    let answer = ask(model = 0, prompt = "What is the meaning of life?");
    println(answer.text);
}
```

```bash
$ cargo run
[AI Call] provider=deepseek, model=deepseek-v4-flash, prompt="What is the meaning of life?"
The meaning of life is a question that has been explored by philosophers...
```

No imports. No JSON wrangling. No retry logic. No token counting. **The language handles the complexity.**

---

## Why AAL?

Every AI application today solves the same problems the same ad-hoc way:

| Problem | Everyone else | AAL |
|---------|--------------|-----|
| **API keys** | Env vars + hope they don't leak | Opaque `api_key` type, compiler-forbidden to print/serialize |
| **Cost control** | Manual `if budget: count++` | `context::new(token_budget = N)` — runtime enforces hard cap |
| **Error handling** | try/except everywhere | `ask.many` — parallel fallbacks, built-in retry |
| **Structured output** | "Return JSON" in prompt + parse | `format = 1` — JSON mode built into the `ask` keyword |
| **Observability** | Manually log each call | Every `ask` auto-instrumented by the runtime |

AAL is not something you import. **It is a language that absorbs complexity so you don't have to.**

---

## Philosophy: Three Pillars

### 少则得 — Less is More
> *"To attain knowledge, add things every day. To attain wisdom, remove things every day."* — Laozi

Only **20 keywords**. A single keyword `ask` replaces hundreds of lines of API boilerplate. The compiler bears the complexity so your code stays readable.

### 法不可违 — The Law Cannot Be Broken
> *"The law is the codification of governance, not to be bent by personal whim."* — Han Feizi

Security and safety are enforced by the compiler and runtime together, never left to developer discretion. An `api_key` value **cannot** be printed, serialized, or leaked — the compiler rejects such code. A `token_budget` **will** stop further AI calls when exhausted — the runtime enforces it.

### 直指人心 — Directly Point to the Mind
> *"A special transmission outside the teachings, not dependent on words or letters."* — Chan Buddhism

`ask` is an expression, not a ritual. `context` transparently manages session state. The language meets you at the level of intent, not implementation.

---

## Quick Start: 5 Minutes

### 1. Build

```bash
git clone https://github.com/gordonlu/aal
cd aal/aal-compiler
cargo build --release
```

### 2. Run with mock (no API key needed)

```bash
AAL_MOCK=1 cargo run
```

### 3. Add an API key

```bash
cargo run -- key add --provider deepseek --key sk-xxx
cargo run -- key list
# => deepseek: sk-5…c221
```

### 4. Go live

```bash
cargo run
```

---

## Model Reference

Models are referenced by numeric code. The mapping is configurable at runtime via environment variables — no recompilation needed.

| Code | Default | Env override |
|------|---------|-------------|
| 0 | DeepSeek V4 Flash | `AAL_MODEL_0=deepseek:deepseek-v4-flash` |
| 1 | DeepSeek V4 Pro | `AAL_MODEL_1=deepseek:deepseek-v4-pro` |
| 2 | OpenAI GPT-4o | `AAL_MODEL_2=openai:gpt-4o` |
| 3 | OpenAI GPT-4o-mini | `AAL_MODEL_3=openai:gpt-4o-mini` |

```bash
# Override any model at runtime
AAL_MODEL_0=openai:gpt-4o cargo run
```

---

## Showcase

### Budget enforcement — "Prevent the disease"

```aal
fn main() {
    let ctx = context::new(token_budget = 1000);
    loop {
        let response = ask(model = 0, context = ctx, prompt = "Tell me a joke", max_tokens = 500);
        println(response.text);
    }
    // After ~2 calls, runtime throws:
    // "token budget exhausted: 1000 used, 1000 budget"
}
```

### Parallel fallback — "Orthodox engages, surprise wins"

```aal
let answers = ask.race([
    (model = 0, prompt = "Classify this text", max_tokens = 50),
    (model = 1, prompt = "Classify this text", max_tokens = 50),
    (model = 2, prompt = "Classify this text", max_tokens = 50),
]);
println(answers[0].text);
```

### Structured output

```aal
let result = ask(model = 0, prompt = "Extract: name, email, age", format = 1);
// result.text contains valid JSON
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
│ Name Resolv │  → Symbol table with built-in types
├─────────────┤
│ Type Check  │  → Bidirectional inference + capability audit
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

Single binary. No VM, no container, no cloud dependency.

---

## CLI

```bash
cargo run -- <file.aal>                          # Compile & run
cargo run -- key add --provider <name> --key <k> # Store API key
cargo run -- key list                            # List keys (masked)
cargo run -- key remove --provider <name>        # Remove key
cargo run -- check <file.aal>                    # Syntax check only

AAL_MOCK=1           cargo run       # Mock mode (no key)
AAL_KEY_DEEPSEEK=sk-xxx cargo run   # Env-var key injection (CI)
AAL_MODEL_0=openai:gpt-4o cargo run # Override model mapping
```

## Environment Variables

| Variable | Purpose |
|----------|---------|
| `AAL_MOCK=1` | Mock mode — returns fake responses, no API key needed |
| `AAL_KEY_<PROVIDER>` | Inject API key by provider (e.g. `AAL_KEY_DEEPSEEK`) |
| `AAL_MODEL_<CODE>` | Override model mapping (e.g. `AAL_MODEL_0=openai:gpt-4o`) |
| `AAL_API_URL` | Override API endpoint (default: OpenAI) |

---

## Project Status

Early prototype. The compiler pipeline is complete and functional.

| Feature | Status |
|---------|--------|
| Lexer + Parser | ✅ Complete |
| Type Checker | ✅ Bidirectional inference + capability audit |
| IR + Lowering | ✅ Complete |
| Interpreter | ✅ Budget enforcement, key injection, mock mode |
| `ask` keyword | ✅ Single, many, race |
| `context` management | ✅ Token budget tracking |
| `api_key` type | ✅ Opaque, compiler-enforced non-leakable |
| Capability system | ✅ `aal.toml` declaration |
| `for` loops | ✅ |
| `if` expressions | ✅ |
| Model mapping | ✅ Env-var configurable, no recompile needed |
| `match` + patterns | ⚠️ Basic (linear chain) |
| Structured output | ⚠️ `format = 1` (JSON mode) |
| Method calls | ⚠️ Flattened to function calls |
| JIT backend | 🔄 Planned (interpreter for now) |
| Package manager | ❌ |
| LSP / IDE support | ❌ |

---

## Contributing

Areas that need love:
- **Language features**: `match` full patterns, struct literals, impl blocks
- **Runtime**: Async streaming, tool registry, actor model
- **Backend**: Cranelift JIT or LLVM
- **Ecosystem**: Package manager, LSP, standard library
- **Documentation**: Grammar reference, tutorials

---

## License

Apache 2.0 — see [LICENSE](LICENSE).

---

*"The best leader is one whose existence is barely known by the people."* — Laozi
