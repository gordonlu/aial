# AIAL (枢言) 快速编程指南 v0.5

本文档为 AI 模型（或人类开发者）提供准确、完整的 AIAL 语言参考。所有函数均通过集成测试验证，非 stub。

---

## 1. 语言基础

### 1.1 Hello World

```aial
fn main() {
    println("Hello, AIAL!");
}
```

运行：
```bash
aial run hello.aal        # 解释执行
aial build hello.aal      # LLVM AOT → aial_output.ll + clang 链接
```

### 1.2 变量与类型

```aial
let x = 42;               // 类型推断 int
let mut y: string = "hi"; // 可变 + 显式类型
y = "hello";
```

| 类型 | 说明 |
|------|------|
| `int` | 64 位有符号整数 |
| `float` | 64 位浮点数 |
| `bool` | true / false |
| `string` | UTF-8 字符串 |
| `api_key` | 不透明密钥类型，不可打印/序列化 |

### 1.3 控制流

```aial
if x > 0 { println("pos"); } else { println("non-pos"); }
while i < 10 { i = i + 1; }
for i in 5 { println(i); }  // 0..4

match response {
    Success => println("ok"),
    Error => println("err"),
}
```

### 1.4 函数定义

```aial
fn add(a: int, b: int) -> int { return a + b; }
fn greet(name: string) -> string { return strcat("Hello, ", name); }
```

### 1.5 泛型

```aial
fn id<T>(x: T) -> T { return x; }
let a = id(42);       // T=int, 生成 id_Int
let b = id("hello");  // T=string, 生成 id_String

struct Container<T> { value: T }
let c = Container { value: 42 };
```

**限制**：多态递归被禁止。

### 1.6 defer

```aial
fn main() {
    defer { println("cleanup"); }
    println("work");
}
// 输出: work, cleanup (LIFO)
```

### 1.7 模块

```aial
module Greetings {
    fn hello() -> string { return "hello"; }
}
let msg = Greetings::hello();
```

### 1.8 include

```aial
include "theme/dark.aal"
fn main() { ... }
```

支持嵌套，循环引用会报错。

---

## 2. AI 核心

### 2.1 ask 关键字

```aial
// 基本
let r = ask(model = 0, context = ctx, prompt = "你好", max_tokens = 256);

// 流式
let stream = ask(model = 0, prompt = "讲个故事", stream = true);
loop {
    let token = ask::read_token(stream);
    if token == "" { break; }
    print(token);
}

// 并行
let answers = ask.many([
    (model = 0, prompt = "A", max_tokens = 50),
    (model = 0, prompt = "B", max_tokens = 50),
]);
```

### 2.2 context 管理

```aial
let ctx = context::new(system_prompt = "你是助手", token_budget = 4096);
ctx = context::add_message(ctx, "user", "hello");
```

### 2.3 工具调用

```aial
#[tool(name = "get_date", description = "获取当前日期")]
fn get_date() -> string { return time::now(); }

fn main() {
    let r = ask(model = 0, prompt = "今天几号？");
}
```

### 2.4 思考模式

DeepSeek-V4 默认启用。thinking/reasoning 内容以灰色显示。

---

## 3. 标准库速查（80+ 函数）

### HTTP (14)
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

### JSON (11)
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

### Map / 哈希表 (5)
```aial
map::new() -> int
map::set(m, key, value) -> void
map::get(m, key) -> string
map::has(m, key) -> bool
map::remove(m, key) -> void
```

### Array / 数组 (5)
```aial
array::new() -> int
array::push(a, value) -> void
array::get(a, index) -> string
array::len(a) -> int
array::sort(a) -> void
```

### Heap / 优先队列 (5)
```aial
heap::new() -> int
heap::push(h, value, priority) -> void
heap::pop(h) -> string
heap::peek(h) -> string
heap::len(h) -> int
```

### Actor / 并发 (7)
```aial
actor::spawn() -> int
actor::spawn_handler(fn_name, init_msg) -> int
actor::send(pid, msg) -> void
actor::recv(pid) -> string
actor::try_recv(pid) -> string
actor::recv_timeout(pid, ms) -> string
actor::error(pid) -> string
```

### Line Editor (4)
```aial
line::new(prompt) -> int
line::read(handle) -> string
line::redraw(handle) -> void
line::end(handle) -> void
```

### IO + Terminal (9)
```aial
print(text) -> void
println(text) -> void
io::readln() -> string
io::readln_timeout(ms) -> string
io::readkey() -> string
io::readkey_timeout(ms) -> string
io::read_multiline() -> string
io::raw_mode(bool) -> void
term::clear() -> void
term::height() -> int
term::scroll_region(top, bottom) -> void
term::redraw(rows) -> void
term::draw_text_clipped(row, col, width, text) -> void
term::cursor_row() -> int
```

### 字符串 (7)
```aial
strlen(s) -> int
strslice(s, start, len) -> string
strcat(a, b) -> string
strchr(s, idx) -> int
str_eq(a, b) -> bool
starts_with(s, prefix) -> bool
str_find(haystack, needle) -> int  // 返回索引，-1=未找到
```

### Context Memory / SQLite (7)
```aial
ctx::open_memory(path) -> int
ctx::save_message(db, session, role, content) -> void
ctx::load_messages(db, session, limit) -> string
ctx::load_messages_since(db, session, timestamp) -> string
ctx::close_memory(db) -> void
ctx::last_error() -> string
```

### File (5)
```aial
file::read(path) -> string
file::write(path, content) -> void
file::append(path, content) -> void
file::patch(path, old, new) -> void
file::list_dir(path) -> string  // 换行分隔
```

### Key Management (3)
```aial
key::set(provider, key) -> int
key::exists(provider) -> int
key::delete(provider) -> int
```

### Process (1)
```aial
process::run(cmd) -> string  // 执行 shell 命令，返回 stdout
```

### FFI (3)
```aial
ffi::load(path) -> int
ffi::call(handle, fn_name, a1..a6) -> int
ffi::close(handle) -> void
```

### Time (3)
```aial
time::sleep(ms) -> void
time::now() -> string         // ISO 8601 日期时间
time::now_ms() -> int         // Unix 毫秒时间戳
```

### Convert (2)
```aial
int_to_string(n) -> string
string_to_int(s) -> int
```

### Other (3)
```aial
token_estimate(text) -> int   // token 数估算
html::escape(text) -> string  // 转义 < > & "
args() -> string              // 命令行参数（换行分隔）
```

---

## 4. 重要约定

1. **`+` 只能整数加法**，字符串拼接用 `strcat(a, b)`
2. **运行时字符串比较用 `str_eq`**，`==` 比较编译期索引
3. **JSON 不 panic**，parse 失败返回 type=-1
4. **HTTP 不抛异常**，错误存在 status=0
5. **终端颜色** 通过 ANSI 转义序列嵌入字符串
6. **aial.toml** 使用序列格式声明 capabilities
7. **heap/array/map 类型跟踪** — `heap::push(h, 42)` 在 `Heap<String>` 上会编译错误
8. **Mock 模式** — `AIAL_MOCK=1` 跳过 API 调用，返回假响应

---

## 5. 自举编译器

```aial
// selfhost/compiler.aal — AIAL 编译器在 AIAL 中实现
fn main() {
    let path = args();
    let src = file::read(path);
    // 词法分析 → 解析 → 生成 LLVM IR → clang 编译 → 运行
}
```

```bash
cd selfhost
aial build compiler.aal
clang aial_output.ll -L ../aial-rt/target/release -laial_rt -lm -lpthread -ldl -rdynamic -o aialc
./aialc hello.aal  # 自举编译
```
