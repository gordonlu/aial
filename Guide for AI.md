# AIAL Programming Guide v0.5

AIAL (枢言) is an AI-native programming language designed to make AI application development a runtime capability instead of a library problem.

This document provides a verified reference for the AIAL language and runtime.

All documented functions are implemented and covered by integration tests unless explicitly marked otherwise.

---

# 1. Language Basics

## 1.1 Hello World

```aial
fn main() {
    println("Hello, AIAL!");
}
````

Run:

```bash
aial run hello.aal
aial build hello.aal
```

`aial build` generates LLVM IR and links it through `clang`.

---

# 1.2 Variables and Types

```aial
let x = 42;
let mut y: string = "hi";

y = "hello";
```

Supported primitive types:

| Type      | Description           |
| --------- | --------------------- |
| `int`     | 64-bit signed integer |
| `float`   | 64-bit floating point |
| `bool`    | Boolean               |
| `string`  | UTF-8 string          |
| `api_key` | Opaque secret type    |

---

# 1.3 Control Flow

```aial
if x > 0 {
    println("positive");
} else {
    println("non-positive");
}

while i < 10 {
    i = i + 1;
}

for i in 5 {
    println(i);
}
```

`for i in 5` iterates from `0..4`.

---

# 1.4 Functions

```aial
fn add(a: int, b: int) -> int {
    return a + b;
}

fn greet(name: string) -> string {
    return strcat("Hello, ", name);
}
```

---

# 1.5 Generics

```aial
fn identity<T>(x: T) -> T {
    return x;
}

let a = identity(42);
let b = identity("hello");

struct Container<T> {
    value: T
}

let c = Container { value: 42 };
```

AIAL uses monomorphization.

Example:

```text
identity<int>    -> identity_Int
identity<string> -> identity_String
```

Recursive polymorphic expansion is forbidden.

---

# 1.6 defer

```aial
fn main() {
    defer {
        println("cleanup");
    }

    println("work");
}
```

Output:

```text
work
cleanup
```

Deferred blocks execute in LIFO order.

---

# 1.7 Modules

```aial
module Greetings {
    fn hello() -> string {
        return "hello";
    }
}

let msg = Greetings::hello();
```

---

# 1.8 include

```aial
include "theme/dark.aal"

fn main() {
    println("loaded");
}
```

Nested includes are supported.

Circular includes produce a compiler error.

---

# 2. AI-Native Features

## 2.1 ask Keyword

AI interaction is a runtime-native capability.

### Basic Usage

```aial
let response = ask(
    model = 0,
    context = ctx,
    prompt = "Hello",
    max_tokens = 256
);
```

---

### Streaming

```aial
let stream = ask(
    model = 0,
    prompt = "Tell me a story",
    stream = true
);

loop {
    let token = ask::read_token(stream);

    if token == "" {
        break;
    }

    print(token);
}
```

---

### Parallel Requests

```aial
let responses = ask.many([
    (model = 0, prompt = "A"),
    (model = 0, prompt = "B")
]);
```

---

# 2.2 Context Management

```aial
let ctx = context::new(
    system_prompt = "You are an assistant",
    token_budget = 4096
);

ctx = context::add_message(
    ctx,
    "user",
    "hello"
);
```

---

# 2.3 Tool Calls

```aial
#[tool(
    name = "get_date",
    description = "Get current date"
)]
fn get_date() -> string {
    return time::now();
}
```

The runtime may automatically invoke tools during `ask`.

---

# 2.4 Reasoning / Thinking Mode

Reasoning-capable models may emit intermediate thinking output.

Thinking output is runtime-controlled and model-dependent.

---

# 3. Standard Library

---

# 3.1 HTTP

```aial
http::get(url) -> int
http::post(url, body) -> int
http::post_json(url, json_val) -> int

http::status(resp) -> int
http::text(resp) -> string

http::header_map() -> int
http::header_set(map, key, val) -> int

http::start(port) -> int
http::listen(handle) -> int
http::respond(req, body, content_type) -> void

http::body(req) -> string
http::method(req) -> string
http::path(req) -> string
http::header(req, key) -> string
```

HTTP APIs do not panic.

Failure is represented through runtime values.

---

# 3.2 JSON

```aial
json::parse(text) -> int
json::stringify(val) -> string

json::get(val, key) -> int
json::get_or(val, key, default) -> int

json::type_of(val) -> int

json::to_string(val) -> string
json::to_int(val) -> int
json::to_float(val) -> float

json::array_len(val) -> int
json::array_get(val, idx) -> int
```

JSON parsing failures never panic.

---

# 3.3 Map

```aial
map::new() -> int

map::set(map, key, value) -> void
map::get(map, key) -> string

map::has(map, key) -> bool
map::remove(map, key) -> void
```

---

# 3.4 Array

```aial
array::new() -> int

array::push(arr, value) -> void
array::get(arr, index) -> string

array::len(arr) -> int
array::sort(arr) -> void
```

---

# 3.5 Heap / Priority Queue

```aial
heap::new() -> int

heap::push(heap, value, priority) -> void
heap::pop(heap) -> string

heap::peek(heap) -> string
heap::len(heap) -> int
```

---

# 3.6 Actors / Concurrency

```aial
actor::spawn() -> int
actor::spawn_handler(fn_name, init_msg) -> int

actor::send(pid, msg) -> void

actor::recv(pid) -> string
actor::try_recv(pid) -> string
actor::recv_timeout(pid, ms) -> string

actor::error(pid) -> string
```

---

# 3.7 IO

```aial
print(text) -> void
println(text) -> void

io::readln() -> string
io::readln_timeout(ms) -> string

io::readkey() -> string
io::readkey_timeout(ms) -> string

io::read_multiline() -> string

io::raw_mode(bool) -> void
io::is_tty() -> int
```

---

# 3.8 Terminal

```aial
term::clear() -> void

term::height() -> int
term::width() -> int

term::scroll_region(top, bottom) -> void
term::reset_scroll_region() -> void

term::redraw(rows) -> void

term::draw_text(row, col, text) -> void
term::draw_text_clipped(row, col, width, text) -> void

term::cursor_goto(row, col) -> void
term::cursor_row() -> int
```

---

# 3.9 Strings

```aial
strlen(s) -> int
strslice(s, start, len) -> string

strcat(a, b) -> string

strchr(s, idx) -> int

str_eq(a, b) -> bool
starts_with(s, prefix) -> bool

str_find(haystack, needle) -> int
```

`str_find` returns:

* index if found
* `-1` if not found

---

# 3.10 Context Memory

```aial
ctx::open_memory(path) -> int

ctx::save_message(db, session, role, content) -> void

ctx::load_messages(db, session, limit) -> string
ctx::load_messages_since(db, session, timestamp) -> string

ctx::close_memory(db) -> void

ctx::last_error() -> string
```

---

# 3.11 Files

```aial
file::read(path) -> string
file::write(path, content) -> void

file::append(path, content) -> void

file::patch(path, old, new) -> void

file::list_dir(path) -> string
```

`list_dir` returns newline-separated entries.

---

# 3.12 Key Management

```aial
key::set(provider, key) -> int
key::exists(provider) -> int
key::delete(provider) -> int
```

---

# 3.13 Process Execution

```aial
process::run(cmd) -> string
```

Executes a shell command and returns stdout.

---

# 3.14 FFI

```aial
ffi::load(path) -> int

ffi::call(handle, fn_name, a1..a6) -> int

ffi::close(handle) -> void
```

---

# 3.15 Time

```aial
time::sleep(ms) -> void

time::now() -> string
time::now_ms() -> int
```

---

# 3.16 Conversion

```aial
int_to_string(n) -> string
string_to_int(s) -> int
```

---

# 3.17 Miscellaneous

```aial
token_estimate(text) -> int

html::escape(text) -> string

args() -> string
```

`args()` returns newline-separated command line arguments.

---

# 4. Language Semantics

## 4.1 Integer Addition vs String Concatenation

`+` is integer-only addition.

String concatenation uses:

```aial
strcat(a, b)
```

---

# 4.2 String Equality

Runtime string equality uses:

```aial
str_eq(a, b)
```

Do not use `==` for runtime string comparison.

---

# 4.3 Error Handling Philosophy

AIAL prioritizes:

* explicit runtime behavior
* deterministic semantics
* non-panicking runtime APIs

JSON parsing errors do not panic.

HTTP failures do not throw exceptions.

Runtime failures are represented through values.

---

# 4.4 UTF-8 Semantics

All AIAL strings are UTF-8.

Important:

```text
strlen() != terminal display width
```

Unicode-aware rendering should use display-width semantics.

---

# 4.5 Runtime Integrity

AIAL discourages fake runtime behavior.

Forbidden:

* hardcoded success paths
* fake FFI implementations
* silent stub behavior

Missing runtime functionality should fail explicitly.

---

# 4.6 Mock Mode

```bash
AIAL_MOCK=1
```

Enables mock AI responses without external API calls.

Useful for:

* testing
* CI
* offline development

---

# 5. Self-Hosting Compiler

The AIAL compiler is implemented in AIAL itself.

Example:

```aial
fn main() {
    let path = args();

    let src = file::read(path);

    // lexer
    // parser
    // LLVM IR generation
    // clang invocation
}
```

Build:

```bash
cd selfhost

aial build compiler.aal

clang aial_output.ll \
    -L ../aial-rt/target/release \
    -laial_rt \
    -lm \
    -lpthread \
    -ldl \
    -rdynamic \
    -o aialc
```

Run:

```bash
./aialc hello.aal
```

AIAL currently uses:

* LLVM IR text generation
* clang for linking
* runtime-assisted execution

---

# 6. Philosophy

AIAL is designed around the idea that AI interaction should be a language-level capability rather than an external framework concern.

Traditional languages push:

* retries
* streaming
* tool orchestration
* context management
* memory persistence

into user code.

AIAL absorbs these concerns into the runtime.

```
