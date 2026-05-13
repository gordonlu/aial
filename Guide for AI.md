# AIAL (枢言) 快速编程指南

本文档为 AI 模型（或人类开发者）提供一份准确、完整的 AIAL 语言参考，以便快速编写 AIAL 程序。内容基于 v0.3.0 实际实现。

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
    Success => println("ok"),
    Degraded => println("degraded"),
    Refused => println("refused"),
    Error => println("error"),
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

### 1.5 泛型 (Generics)

```aial
// 函数泛型
fn id<T>(x: T) -> T {
    return x;
}
let a = id(42);       // T = int, 生成 id_Int
let b = id("hello");  // T = string, 生成 id_String

// 结构体泛型
struct Container<T> { value: T }
let c = Container { value: 42 };  // T = int
```

**限制**：多态递归被禁止——不能用不同的类型参数递归调用自身。类型系统会捕获此错误。

### 1.6 defer 语句

```aial
fn main() {
    defer { println("cleanup 2"); }
    defer { println("cleanup 1"); }
    println("work");
}
// 输出：work, cleanup 1, cleanup 2（LIFO 执行）
```

### 1.7 模块系统 (Module)

```aial
module Greetings {
    fn hello() -> string {
        return "hello from module";
    }
}

fn main() {
    let msg = Greetings::hello();  // "hello from module"
}
```

模块支持嵌套。函数通过 `ModuleName::funcName` 调用。

### 1.8 include 预处理（文本级拼接）

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

### 2.3 Token 估算

```aial
let tokens = token_estimate("Hello world");  // 粗略估算 token 数（英文 ~4 字符/token）
```

---

## 3. 标准库速查（50+ 函数，全部已实现）

### HTTP (11)

```aial
http::get(url) -> int
http::post(url, body) -> int
http::post_json(url, json_val) -> int
http::status(resp) -> int
http::text(resp) -> string
http::header_map() -> int
http::header_set(map, key, val) -> int
http::start(port) -> int
http::listen(handle) -> int          // 超时返回 -1
http::respond(req, body, content_type) -> void
http::body(req) -> string
http::method(req) -> string
http::path(req) -> string
http::url(req) -> string
http::query(req, key) -> string
http::header(req, key) -> string
http::status_text(code) -> string
http::ok(req, body) -> void
http::json(req, body) -> void
http::html(req, body) -> void
http::serve(req, path) -> void
```

### JSON (9)

```aial
json::parse(text) -> int             // 解析失败返回 type=-1
json::stringify(val) -> string       // JsonValue → JSON 文本
json::get(val, key) -> int           // 缺 key 返回 Null (type=0)
json::get_or(val, key, default) -> int
json::type_of(val) -> int            // 0=null,1=bool,2=number,3=string,4=array,5=object
json::to_string(val) -> string
json::to_int(val) -> int
json::to_float(val) -> float
json::array_len(val) -> int
json::array_get(val, idx) -> int
```

### Map / 哈希表 (5)

```aial
map::new() -> int                    // 创建 Map handle
map::set(m, key, value) -> void
map::get(m, key) -> string           // 不存在返回 ""
map::has(m, key) -> bool
map::remove(m, key) -> void
```

### Array / 数组 + 排序 (5)

```aial
array::new() -> int                  // 创建数组 handle
array::push(a, value) -> void
array::get(a, index) -> string
array::len(a) -> int
array::sort(a) -> void               // 原地按字母排序
```

### Heap / 优先队列 (5)

```aial
heap::new() -> int                   // 创建堆 handle
heap::push(h, value, priority) -> void
heap::pop(h) -> string               // 弹出最高优先级
heap::peek(h) -> string              // 查看最高优先级（不移除）
heap::len(h) -> int
```

### Actor / 并发 (7)

```aial
actor::spawn() -> int                // 创建 actor，返回 pid
actor::spawn_handler(fn_name, init_msg) -> int  // 线程化 actor
actor::send(pid, msg) -> void
actor::recv(pid) -> string           // 阻塞接收
actor::try_recv(pid) -> string       // 非阻塞，空返回 ""
actor::recv_timeout(pid, ms) -> string  // 超时返回 ""
actor::error(pid) -> string          // 获取最后错误信息
```

### IO (5)

```aial
print(text) -> void                  // 无换行输出
println(text) -> void                // 带换行输出
io::readln() -> string               // 阻塞读一行
io::readln_timeout(ms) -> string
io::readkey() -> string              // 读单字符
io::readkey_timeout(ms) -> string
io::raw_mode(bool) -> void           // 终端 raw 模式切换
```

### 字符串 (8)

```aial
strlen(s) -> int
strslice(s, start, len) -> string
strcat(a, b) -> string
strchr(s, idx) -> int                // 第 idx 个字符的码点
str_eq(a, b) -> bool
starts_with(s, prefix) -> bool
```

### Context Memory / SQLite (6)

```aial
ctx::open_memory(path) -> int
ctx::save_message(db, session, role, content) -> void
ctx::load_messages(db, session, limit) -> string
ctx::load_messages_since(db, session, timestamp) -> string
ctx::close_memory(db) -> void
ctx::last_error() -> string
```

**注意**：`ctx::save_message` 需要 4 个参数（db, session, role, content）。

### Time (1)

```aial
time::sleep(ms) -> void
```

### HTML (1)

```aial
html::escape(text) -> string         // 转义 < > & "
```

### FFI (3) — 需要 aial.toml 声明 `unsafe_ffi = true`

```aial
ffi::load(path) -> int
ffi::call(handle, fn_name, args...) -> int
ffi::close(handle) -> void
```

### AI Streaming (1)

```aial
ask::read_token(stream_handle) -> string  // 返回 "" 表示结束
```

### File (4)

```aial
file::read(path) -> string
file::write(path, content) -> void
file::append(path, content) -> void
file::patch(path, old, new) -> void  // 字符串替换
```

### 类型安全增强

heap/array/map 的返回值类型会跟踪元素类型。例如：
```aial
let h = heap::new();
heap::push(h, "hello", 1);   // heap 类型参数推断为 string
let val = heap::pop(h);      // val: string ✓
heap::push(h, 42, 2);       // 编译错误：type mismatch Int vs String
```

---

## 4. 重要约定与限制

1. **`+` 只能做整数加法，不能拼接字符串**——拼接用 `strcat(a, b)`。

2. **`==` 对字符串比较的是内部表索引**——编译期字面量 `"foo" == "foo"` 正确（同索引），但运行时字符串与字面量比较必须用 `str_eq(a, b)`。

3. **JSON 不崩溃**：`json::parse` 返回的 handle，type=-1 表示错误。取字段用 `json::get(val, key)`，缺 key 返回 type=0 (Null)。

4. **HTTP 不抛异常**：`http::get/post` 错误存在 status=0。

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
