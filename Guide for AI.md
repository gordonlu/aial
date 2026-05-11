# AIAL (枢言) 快速编程指南

本文档为 AI 模型（或人类开发者）提供一份准确、完整的 AIAL 语言参考，以便快速编写 AIAL 程序。内容基于 v0.2.0 实际实现。

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
aial build hello.aal      # LLVM AOT 编译为原生二进制
clang aial_output.ll -L ../aial-rt/target/release -laial_rt -lm -lpthread -ldl -o hello
```

### 1.2 变量与类型

```aial
let x = 42;               // 类型推断 int
let mut y: string = "hi"; // 可变 + 显式类型
y = "hello";
```

| 类型 | 说明 | 示例 |
|------|------|------|
| `int` | 64 位有符号整数 | `42` |
| `float` | 64 位浮点数 | `3.14` |
| `bool` | 布尔值 | `true` / `false` |
| `string` | UTF-8 字符串 | `"hello"` |
| `null` | 空值 | `null` |
| `api_key` | 不透明密钥类型，**不可打印、不可序列化** | |

### 1.3 控制流

```aial
// if 语句（也可是表达式）
if x > 0 {
    println("positive");
} else {
    println("non-positive");
}

// while 循环
while i < 10 {
    i = i + 1;
}

// for 循环（计数）
for i in 5 {
    println(i);  // 0, 1, 2, 3, 4
}

// match 穷尽匹配（编译器强制覆盖所有分支）
match response {
    Success(text, _) => println(text),
    Degraded(text, ..) => println(text),
    Refused(reason) => println(reason),
    Error(msg) => println(msg),
}
```

### 1.4 函数定义

```aial
fn add(a: int, b: int) -> int {
    return a + b;
}

fn greet(name: string) -> string {
    return strcat("Hello, ", name);
}
```

### 1.5 include 预处理（文本级拼接）

```aial
include "theme/dark.aal"
include "engines/chat.aal"

fn main() { ... }
```

路径相对当前源文件目录。支持嵌套，循环引用会报错。

---

## 2. AI 核心

### 2.1 ask 关键字

```aial
// 基本形式
let r = ask(model = 0, context = ctx, prompt = "你好", max_tokens = 256);

// 并行调用
let answers = ask.many([
    (model = 0, prompt = "北京天气", max_tokens = 50),
    (model = 0, prompt = "上海天气", max_tokens = 50),
]);

// 流式输出
let stream = ask(model = 0, prompt = "讲个故事", stream = true, max_tokens = 512);
loop {
    let token = ask::read_token(stream);
    if token == "" { break; }
    print(token);
}
```

### 2.2 context 管理

```aial
let ctx = context::new(token_budget = 4096);
let remaining = context::budget(ctx);  // 剩余 token
```

---

## 3. 标准库速查（35 函数，全部已实现）

### HTTP (11)

```aial
http::get(url) -> HttpResponse handle
http::post(url, body) -> HttpResponse handle
http::post_json(url, json_val) -> HttpResponse handle
http::status(resp) -> int
http::text(resp) -> string
http::header_map() -> HeaderMap handle
http::header_set(map, key, val) -> HeaderMap handle
http::start(port) -> ServerHandle
http::listen(handle) -> Request handle   // 超时返回 -1
http::respond(req, body, content_type) -> void
http::body(req) -> string
```

### JSON (9)

```aial
json::parse(text) -> JsonValue handle     // 解析失败返回 type=-1
json::stringify(val) -> string            // JsonValue → JSON 文本
json::get(val, key) -> JsonValue          // 缺 key 返回 Null (type=0)
json::get_or(val, key, default) -> JsonValue
json::to_string(val) -> string            // 提取字符串值
json::to_int(val) -> int                  // 提取整数值
json::to_float(val) -> float              // 提取浮点值
json::array_len(val) -> int
json::array_get(val, idx) -> JsonValue
```

### IO (4)

```aial
print(text)                    // 无换行输出
println(text)                  // 带换行输出
io::readln() -> string         // 阻塞读一行
io::readln_timeout(ms) -> string
io::readkey() -> string        // 读单字符
io::raw_mode(bool) -> void     // 终端 raw 模式切换
```

### 字符串 (8)

```aial
strlen(s) -> int
strslice(s, start, len) -> string
strcat(a, b) -> string
strchr(s, idx) -> int          // 第 idx 个字符的码点
str_eq(a, b) -> bool
starts_with(s, prefix) -> bool
```

### Context Memory (5)

```aial
ctx::open_memory(path) -> db handle
ctx::save_message(db, session, role, content) -> void
ctx::load_messages(db, session, limit) -> JSON string
ctx::load_messages_since(db, session, timestamp) -> JSON string
ctx::close_memory(db) -> void
```

**注意**：`ctx::save_message` 需要 4 个参数（db, session, role, content），不是 2 个。

### Time (1)

```aial
time::sleep(ms) -> void
```

### HTML (1)

```aial
html::escape(text) -> string   // 转义 < > & " 防止 XSS
```

### FFI (3) — 需要 aial.toml 声明 `unsafe_ffi = true`

```aial
ffi::load(path) -> lib handle
ffi::call(handle, fn_name, args...) -> i64
ffi::close(handle) -> void
```

### AI Streaming (1)

```aial
ask::read_token(stream_handle) -> string   // 返回 "" 表示结束
```

### 文件 (已实现)

```aial
file::read(path) -> string      // 读取文件
file::write(path, content) -> void
```

---

## 4. 重要约定与限制

1. **`+` 只能做整数加法，不能拼接字符串**——拼接用 `strcat(a, b)`。

2. **`==` 对字符串比较的是内部表索引**——编译期字面量 `"foo" == "foo"` 正确（同索引），但运行时字符串（`io::readln()` 的返回值）与字面量比较必须用 `str_eq(a, b)`。其他类型（int、bool）的 `==` 正常工作。

3. **JSON 不崩溃**：`json::parse` 返回的 handle，type=-1 表示错误。取字段用 `json::get(val, key)`，缺 key 返回 type=0 (Null)。

4. **HTTP 不抛异常**：`http::get/post` 错误存在 status=0，文本存在 body 里。

5. **终端颜色**：通过 ANSI 转义序列嵌入字符串——`"\x1b[32m绿色\x1b[0m"`。

6. **aial.toml**：`allow_network` 和 `allow_filesystem` 用序列格式（非布尔值）：
```toml
[capabilities]
allow_network = [{ provider = "deepseek", models = ["deepseek-v4-flash"] }]
allow_filesystem = [{ path = ".", access = "write" }]
```

---

## 5. 实际可运行的 TUI 模式

```aial
fn main() {
    let db = ctx::open_memory("chat.db");
    let ctx = context::new(token_budget = 32768);
    loop {
        print("> ");
        let input = io::readln();
        if str_eq(input, "/quit") { break; }
        let stream = ask(model = 0, context = ctx, prompt = input, stream = true);
        print("\x1b[36mAI: \x1b[0m");
        loop {
            let token = ask::read_token(stream);
            if token == "" { break; }
            print(token);
        }
        println("");
        ctx::save_message(db, "main", "user", input);
    }
    ctx::close_memory(db);
}
```

---

现在你已经掌握了 AIAL 的全部实际可用的核心概念和标准库。
