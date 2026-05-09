# AIAL Standard Library

## `context` module

### `context::new(...) -> Context`
Create a new session context.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `system_prompt` | `string` | `""` | System prompt for the AI |
| `token_budget` | `int` | `4096` | Hard cap on token usage |
| `strategy` | `string` | `""` | Context window strategy (`""` or `"sliding_window"`) |
| `window_size` | `int` | `0` | Window size for sliding_window strategy |

```aal
let ctx = context::new(
    system_prompt = "you are helpful",
    token_budget = 4096
);
```

### `context::budget(ctx: Context) -> int`
Return remaining token budget. Runtime-enforced — once exhausted, subsequent `ask` calls fail.

### `context::forget(ctx: Context, cause_id: int)`
Causal pruning: remove a message and all messages derived from it. Useful for GDPR / right-to-be-forgotten.

### `context::reflect(ctx: Context) -> string`
Generate a self-correction prompt based on recent interaction history in the causal DAG.

---

## `privacy` module

### `privacy::sensitive(value) -> value`
Mark a value as sensitive (taint tracking). The runtime warns when sensitive data reaches `println` or is passed to `ask` without explicit masking.

```aal
let name = privacy::sensitive(user.name);
let prompt = "Hello, " + name;  // compiler tracks taint
let r = ask(prompt);            // runtime warns: sensitive data in prompt
println(name);                  // runtime warns: printing sensitive data
```

---

## `println`

### `println(value)`
Print a string. The value must not be `api_key` type — the compiler rejects such code at compile time.

---

## `ask` keyword

### `ask([prompt], model, context, temperature, max_tokens, format) -> AiResponse`
Make an AI call. The first positional argument (if bare, without `name =`) is treated as `prompt`.

### `ask.many([(model, prompt, ...), ...]) -> AiResponse[]`
Parallel AI calls. Each group runs concurrently. Returns an array of responses.

### `ask.race([(model, prompt, ...), ...]) -> AiResponse`
Race AI calls. Returns the first successful response.

---

## `AiResponse` type

Four variants (exhaustive match required):

| Variant | Fields | Meaning |
|---------|--------|---------|
| `Success` | `text, usage` | Normal completion |
| `Degraded` | `text, reason, usage` | Completed with reduced quality |
| `Refused` | `reason` | Model refused to answer |
| `Error` | `error` | Runtime or API error |
