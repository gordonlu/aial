# AIAL (枢言) — Code Wiki
## 目录

1. [项目概述](#1-项目概述)
2. [项目整体架构](#2-项目整体架构)
3. [编译流水线 (Pipeline)](#3-编译流水线-pipeline)
4. [模块详解 — aial-compiler](#4-模块详解--aial-compiler)
   - [4.1 main.rs — CLI 入口与命令分发](#41-mainrs--cli-入口与命令分发)
   - [4.2 token.rs — Token 定义](#42-tokenrs--token-定义)
   - [4.3 lexer.rs — 词法分析器](#43-lexerrs--词法分析器)
   - [4.4 ast.rs — 抽象语法树](#44-astrs--抽象语法树)
   - [4.5 parser.rs — 语法解析器](#45-parserrs--语法解析器)
   - [4.6 types.rs — 类型系统定义](#46-typesrs--类型系统定义)
   - [4.7 symbol.rs — 符号表](#47-symbolrs--符号表)
   - [4.8 type_checker.rs — 类型检查器](#48-type_checkerrs--类型检查器)
   - [4.9 ir.rs — 中间表示 (IR)](#49-irrs--中间表示-ir)
   - [4.10 ir_builder.rs — AST → IR 转换器](#410-ir_builderrs--ast--ir-转换器)
   - [4.11 ir_lower.rs — IR 降级与运行时函数注册](#411-ir_lowerrs--ir-降级与运行时函数注册)
   - [4.12 interpreter.rs — IR 解释器](#412-interpreterrs--ir-解释器)
   - [4.13 llvm_backend.rs — LLVM 后端](#413-llvm_backendrs--llvm-后端)
   - [4.14 capability.rs — 能力声明系统](#414-capabilityrs--能力声明系统)
   - [4.15 key_manager.rs — API Key 管理](#415-key_managerrs--api-key-管理)
   - [4.16 philosophy.rs — 诊断风格引擎](#416-philosophyrs--诊断风格引擎)
5. [模块详解 — aial-rt (运行时库)](#5-模块详解--aial-rt-运行时库)
   - [5.1 全局状态管理](#51-全局状态管理)
   - [5.2 AI 调用 (ask)](#52-ai-调用-ask)
   - [5.3 HTTP 客户端与服务端](#53-http-客户端与服务端)
   - [5.4 JSON 解析与操作](#54-json-解析与操作)
   - [5.5 上下文管理 (Context)](#55-上下文管理-context)
   - [5.6 Actor 模型](#56-actor-模型)
   - [5.7 I/O 与终端](#57-io-与终端)
   - [5.8 数据结构 (Map, Heap, Array)](#58-数据结构-map-heap-array)
   - [5.9 FFI (外部函数接口)](#59-ffi-外部函数接口)
   - [5.10 其他函数](#510-其他函数)
6. [模块详解 — aial-vscode](#6-模块详解--aial-vscode)
7. [模块详解 — selfhost](#7-模块详解--selfhost)
8. [模块详解 — docs](#8-模块详解--docs)
9. [依赖关系](#9-依赖关系)
10. [项目运行方式](#10-项目运行方式)
11. [示例代码与测试](#11-示例代码与测试)

---

## 1. 项目概述

AIAL（枢言）是一个 **AI-native** 编程语言，`ask` 是第一类关键字（first-class keyword）。这意味着在 AIAL 中，调用 AI 模型就像调用普通函数一样自然。语言提供了完整的编译工具链，支持解释执行和 LLVM AOT 编译两种模式。

**核心特性**：
- `ask` 关键字：内建 AI 调用，支持模型选择、流式响应、上下文管理和工具调用
- 泛型 (Generics)：支持 `fn id<T>(x: T) -> T` 风格的泛型函数与结构体
- 模块系统：`module Name { fn ... }` 代码组织，支持嵌套模块
- Actor 模型：`actor::spawn/send/recv/try_recv/recv_timeout`
- `defer` 语句：LIFO 清理块
- Match 穷尽性检查：编译器强制覆盖所有枚举变体
- LLVM AOT 编译：通过 clang 链接生成本地二进制
- 自举 (Self-hosting)：用 AIAL 编写的 AIAL 编译器
- 80+ 标准库函数

---

## 2. 项目整体架构

```
aial/
├── aial-compiler/          # 编译器 (Rust) — 核心模块
│   ├── main.rs             # CLI 入口，命令分发 (run/build/key)
│   ├── token.rs            # Token 类型定义
│   ├── lexer.rs            # 词法分析器
│   ├── ast.rs              # AST 节点定义
│   ├── parser.rs           # 递归下降解析器
│   ├── types.rs            # 类型系统定义
│   ├── symbol.rs           # 符号表
│   ├── type_checker.rs     # 类型检查器（含泛型单态化）
│   ├── ir.rs               # 中间表示 (IR) 定义
│   ├── ir_builder.rs       # AST → IR 转换器
│   ├── ir_lower.rs         # IR 降级与运行时函数注册
│   ├── interpreter.rs      # IR 解释器 (开发模式)
│   ├── llvm_backend.rs     # LLVM IR 文本生成器
│   ├── capability.rs       # 能力声明系统 (aial.toml)
│   ├── key_manager.rs      # API Key 安全管理
│   ├── philosophy.rs       # 诊断风格引擎 (tao/legalist/medical)
│   ├── examples/           # 示例 AAL 程序
│   └── tests/              # 集成测试
├── aial-rt/                # 运行时库 (Rust staticlib)
│   └── src/lib.rs          # C ABI 运行时函数实现
├── aial-vscode/            # VS Code 语法高亮扩展
├── selfhost/               # 自举编译器 (AIAL 语言)
│   └── compiler.aal        # AIAL 编写的 LLVM IR 编译器
├── docs/                   # 语言规范文档
│   ├── GRAMMAR.md          # 语法规范
│   ├── IR.md               # IR 文档
│   ├── PHILOSOPHY.md       # 设计哲学
│   ├── STDLIB.md           # 标准库文档
│   └── TYPE_SYSTEM.md      # 类型系统文档
├── build.sh                # 构建脚本
├── README.md               # 项目说明
└── Guide for AI.md         # AI 使用指南
```

**架构数据流**：

```
源文件 (.aal)
    │
    ▼
[Lexer] ──► Token 流
    │
    ▼
[Parser] ──► AST (抽象语法树)
    │
    ▼
[Type Checker] ──► 类型化 AST + 符号表 + 泛型单态化
    │
    ▼
[IR Builder] ──► IR Module (中间表示)
    │
    ├──► [Interpreter] ──► 直接执行 (aial run)
    │
    └──► [IR Lower] ──► [LLVM Backend] ──► .ll 文件 ──► clang ──► 二进制 (aial build)
```

---

## 3. 编译流水线 (Pipeline)

AIAL 编译器采用经典的 **前端 → 中端 → 后端** 架构：

### 3.1 前端 (Frontend)

| 阶段 | 模块 | 输入 | 输出 |
|------|------|------|------|
| 词法分析 | [lexer.rs](file:///workspace/aial/aial-compiler/lexer.rs) | 源代码字符串 | Token 流 |
| 语法分析 | [parser.rs](file:///workspace/aial/aial-compiler/parser.rs) | Token 流 | AST (Program) |
| 类型检查 | [type_checker.rs](file:///workspace/aial/aial-compiler/type_checker.rs) | AST | 类型化 AST + 泛型特化信息 |

### 3.2 中端 (Middle-end)

| 阶段 | 模块 | 输入 | 输出 |
|------|------|------|------|
| IR 构建 | [ir_builder.rs](file:///workspace/aial/aial-compiler/ir_builder.rs) | AST + 类型信息 | IR Module |
| IR 降级 | [ir_lower.rs](file:///workspace/aial/aial-compiler/ir_lower.rs) | IR Module | 降级后的 IR + 运行时函数注册表 |

### 3.3 后端 (Backend)

| 模式 | 模块 | 说明 |
|------|------|------|
| 解释执行 | [interpreter.rs](file:///workspace/aial/aial-compiler/interpreter.rs) | 直接解释 IR 指令，适合开发调试 |
| LLVM AOT | [llvm_backend.rs](file:///workspace/aial/aial-compiler/llvm_backend.rs) | 生成 LLVM IR 文本，由 clang 编译为二进制 |

---

## 4. 模块详解 — aial-compiler

### 4.1 main.rs — CLI 入口与命令分发

**路径**: [main.rs](file:///workspace/aial/aial-compiler/main.rs)

`main.rs` 是编译器的 CLI 入口，使用 `clap` 解析命令行参数，支持以下子命令：

| 命令 | 功能 |
|------|------|
| `aial run <file>` | 解释执行 .aal 文件 |
| `aial build <file>` | LLVM AOT 编译为 .ll + 二进制 |
| `aial key add/list/remove` | API Key 管理 |
| `aial doc <module>` | 标准库文档查询 |

**核心流程 (run 命令)**:
1. 加载配置文件 `aial.toml` (能力声明)
2. 读取源文件
3. 词法分析 (Lexer) → Token 流
4. 语法分析 (Parser) → AST
5. 类型检查 (TypeChecker) → 类型化 AST + 泛型特化
6. IR 构建 (IRBuilder) → IR Module
7. IR 降级 (IR Lower) → 运行时函数注册
8. 解释执行 (Interpreter) → 输出结果

**核心流程 (build 命令)**:
1-6 同上
7. IR 降级
8. LLVM 代码生成 → `.ll` 文件
9. 调用 `clang` 编译 + 链接 `libaial_rt.a` → 二进制

**特殊参数**:
- `--philosophy tao|legalist|medical`: 切换诊断风格
- `AIAL_MOCK=1`: 启用 Mock 模式（不调用真实 API）

---

### 4.2 token.rs — Token 定义

**路径**: [token.rs](file:///workspace/aial/aial-compiler/token.rs)

定义了词法分析器输出的所有 Token 类型，包括：

**关键 Token 枚举**:

| 类别 | Token | 说明 |
|------|-------|------|
| 关键字 | `Fn`, `Let`, `If`, `Else`, `Match`, `While`, `For`, `Loop`, `Return`, `Break`, `Continue`, `Defer`, `Module`, `Struct`, `Enum`, `Self_`, `Ask`, `AskMany`, `AskRace`, `As`, `True`, `False`, `Null` | 语言关键字 |
| 字面量 | `Int(i64)`, `Float(f64)`, `String(String)` | 常量值 |
| 标识符 | `Ident(String)` | 变量名、函数名 |
| 运算符 | `Plus`, `Minus`, `Star`, `Slash`, `EqEq`, `NotEq`, `Lt`, `Gt`, `Le`, `Ge`, `AndAnd`, `OrOr`, `Not` | 二元/一元运算符 |
| 分隔符 | `LParen`, `RParen`, `LBrace`, `RBrace`, `LBracket`, `RBracket`, `Comma`, `Colon`, `Semi`, `Arrow`, `Dot`, `FatArrow`, `Pipe`, `At` | 语法分隔符 |

---

### 4.3 lexer.rs — 词法分析器

**路径**: [lexer.rs](file:///workspace/aial/aial-compiler/lexer.rs)

手写词法分析器，核心结构为 `Lexer` 结构体：

```rust
pub struct Lexer {
    source: Vec<char>,
    pos: usize,
}
```

**关键方法**:
- `new(source: &str) -> Self`: 创建词法分析器实例
- `next_token() -> Token`: 读取下一个 Token
- `tokenize() -> Vec<Token>`: 一次性获取所有 Token
- `skip_whitespace()`: 跳过空白字符
- `read_number() -> Token`: 解析整数/浮点数
- `read_string() -> Token`: 解析字符串字面量
- `read_ident() -> Token`: 解析标识符或关键字

**特点**: 使用 `Span` 记录每个 Token 的源码位置 (start, end)，用于错误报告。

---

### 4.4 ast.rs — 抽象语法树

**路径**: [ast.rs](file:///workspace/aial/aial-compiler/ast.rs)

定义了完整的 AST 节点类型体系：

**顶层结构**:
```rust
pub struct Program {
    pub items: Vec<TopLevelItem>,
    pub main_fn: Option<FnDef>,
}
```

**TopLevelItem 枚举**:
| 变体 | 说明 |
|------|------|
| `FnDef(FnDef)` | 函数定义 |
| `Module(Module)` | 模块定义 |
| `StructDef(StructDef)` | 结构体定义 |
| `EnumDef(EnumDef)` | 枚举定义 |
| `UseStmt(UseStmt)` | 导入语句 |
| `Test(FnDef)` | 测试函数 |

**Stmt (语句) 枚举**:
| 变体 | 说明 |
|------|------|
| `Let(LetStmt)` | 变量声明 |
| `Assign(AssignStmt)` | 赋值语句 |
| `Expression(Expr)` | 表达式语句 |
| `Return(Option<Expr>)` | 返回语句 |
| `If(IfStmt)` | 条件语句 |
| `Match(MatchStmt)` | 模式匹配 |
| `For(ForStmt)` | For 循环 |
| `While(WhileStmt)` | While 循环 |
| `Loop(LoopStmt)` | 无限循环 |
| `Break/Span` | break 语句 |
| `Continue/Span` | continue 语句 |
| `Defer(Block)` | defer 延迟执行块 |

**ExprKind (表达式) 枚举**:
| 变体 | 说明 |
|------|------|
| `IntLiteral(i64)` | 整数字面量 |
| `FloatLiteral(f64)` | 浮点字面量 |
| `StringLiteral(String)` | 字符串字面量 |
| `BoolLiteral(bool)` | 布尔字面量 |
| `NullLiteral` | null 字面量 |
| `Variable(Ident)` | 变量引用 |
| `SelfExpr` | self 表达式 |
| `Unary(UnOp, Box<Expr>)` | 一元运算 |
| `Binary(BinOp, Box<Expr>, Box<Expr>)` | 二元运算 |
| `Call(Box<Expr>, Vec<Expr>, Vec<NamedArg>)` | 函数调用 |
| `FieldAccess { receiver, field }` | 字段访问 |
| `MethodCall { receiver, method, generic_args, args }` | 方法调用 |
| `Index(Box<Expr>, Box<Expr>)` | 索引访问 |
| `StructLiteral { struct_name, fields }` | 结构体字面量 |
| `IfExpr(Box<Expr>, Block, Box<Expr>)` | if 表达式 |
| `MatchExpr(Box<Expr>, Vec<MatchArm>)` | match 表达式 |
| `BlockExpr(Block)` | 块表达式 |
| `Ask(Vec<AskOption>)` | AI ask 调用 |
| `AskMany(Vec<Vec<AskOption>>)` | 并行 ask 调用 |
| `AskRace(Vec<Vec<AskOption>>)` | 竞速 ask 调用 |
| `Receive` | Actor 接收表达式 |
| `Path(Path)` | 路径表达式 (如 `std::io::readln`) |
| `Pipe(Box<Expr>, Box<Expr>)` | 管道运算符 `\|>` |

---

### 4.5 parser.rs — 语法解析器

**路径**: [parser.rs](file:///workspace/aial/aial-compiler/parser.rs)

递归下降解析器，`Parser` 结构体封装了解析状态：

```rust
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}
```

**关键方法**:

| 方法 | 功能 |
|------|------|
| `parse() -> Result<Program>` | 顶层入口，解析整个程序 |
| `parse_item() -> TopLevelItem` | 解析顶层项 (fn/module/struct/enum/use/test) |
| `parse_fn_def() -> FnDef` | 解析函数定义 |
| `parse_module() -> Module` | 解析模块定义 |
| `parse_stmt() -> Stmt` | 解析语句 |
| `parse_expr() -> Expr` | 解析表达式 (含 Pratt 解析器处理优先级) |
| `parse_block() -> Block` | 解析代码块 `{ ... }` |
| `parse_pattern() -> Pattern` | 解析模式 (用于 match) |
| `parse_type() -> Type` | 解析类型注解 |

**表达式优先级 (Pratt Parsing)**:
1. 字面量 / 变量 / 路径
2. 函数调用 / 方法调用 / 字段访问 / 索引
3. 一元运算符 (`-`, `!`)
4. 乘除取余 (`*`, `/`, `%`)
5. 加减 (`+`, `-`)
6. 比较运算符 (`==`, `!=`, `<`, `>`, `<=`, `>=`)
7. 逻辑与 (`&&`)
8. 逻辑或 (`||`)
9. 管道 (`|>`)
10. if 表达式

**特殊解析**:
- `ask` 关键字：解析命名参数 `ask(model=0, prompt="...")`
- `ask.many` / `ask.race`：解析并行/竞速 AI 调用
- `#[tool]` 属性：解析函数属性注解
- 泛型参数：`fn foo<T>(x: T) -> T`

---

### 4.6 types.rs — 类型系统定义

**路径**: [types.rs](file:///workspace/aial/aial-compiler/types.rs)

定义了 AIAL 语言的全部类型：

```rust
pub enum Type {
    Base(BaseType),
    Generic(String),              // T (泛型类型参数)
    Function(Vec<Type>, Box<Type>),  // 函数类型
    OpaqueStruct(Ident, Vec<Type>), // 不透明结构体
    Array(Box<Type>),             // 数组类型
    Named(Ident),                 // 命名类型
}

pub enum BaseType {
    Int, Int64, Int32,
    Float, Float64,
    Bool, String,
    Void,
}
```

**TypeEnv (类型环境)**: 管理类型上下文，用于类型推断和泛型特化。

---

### 4.7 symbol.rs — 符号表

**路径**: [symbol.rs](file:///workspace/aial/aial-compiler/symbol.rs)

符号表 `SymbolTable` 管理作用域内的符号信息：

```rust
pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
}
```

**Symbol 结构体**: 记录符号的名称、类型、可变性等信息。

**关键方法**:
- `push_scope()` / `pop_scope()`: 管理嵌套作用域
- `insert(name, symbol)`: 在当前作用域插入符号
- `lookup(name)`: 从内到外查找符号

---

### 4.8 type_checker.rs — 类型检查器

**路径**: [type_checker.rs](file:///workspace/aial/aial-compiler/type_checker.rs)

`TypeChecker` 负责类型检查与推断，以及泛型函数的单态化 (monomorphization)。

**核心功能**:
1. **类型检查**: 验证表达式类型一致性
2. **类型推断**: 自动推断变量和表达式类型
3. **泛型单态化**: 检测泛型函数的所有实例化调用，生成特化信息 (specializations)
4. **Match 穷尽性检查**: 确保 match 覆盖所有枚举变体
5. **能力检查**: 验证 AI provider/model 是否在 `aial.toml` 中声明
6. **Lint 检查**: 未使用的 match 变量警告、静默错误丢弃警告

**泛型处理流程**:
1. 遍历 AST，收集泛型函数定义
2. 遍历所有函数调用，匹配泛型参数的实际类型
3. 生成 `specializations: HashMap<fn_name, HashMap<type_params, mangled_name>>`
4. 传递给 IR Builder 用于生成单态化后的 IR 函数

---

### 4.9 ir.rs — 中间表示 (IR)

**路径**: [ir.rs](file:///workspace/aial/aial-compiler/ir.rs)

定义了编译器内部使用的中间表示。IR 是 SSA 风格的低级表示，位于 AST 和 LLVM IR 之间。

**核心数据结构**:

```rust
pub struct IRModule {
    pub functions: Vec<IRFunction>,
    pub strings: Vec<String>,
    pub tool_registrations: Vec<ToolRegistration>,
}

pub struct IRFunction {
    pub name: String,
    pub params: Vec<(Value, IRType)>,
    pub return_type: IRType,
    pub blocks: Vec<BasicBlock>,
    pub entry: BlockId,
    pub value_types: Vec<(Value, IRType)>,
}

pub struct BasicBlock {
    pub id: BlockId,
    pub instrs: Vec<(Instr, Option<Value>)>,
    pub terminator: Option<Terminator>,
}
```

**Instr (指令) 枚举**:

| 指令 | 说明 |
|------|------|
| `ConstInt(i64)` | 整数常量 |
| `ConstFloat(f64)` | 浮点常量 |
| `ConstBool(bool)` | 布尔常量 |
| `ConstNull` | Null 常量 |
| `ConstString(String)` | 字符串常量 |
| `BinOp(BinOp, Value, Value)` | 二元运算 |
| `UnOp(UnOp, Value)` | 一元运算 |
| `Cmp(BinOp, Value, Value)` | 比较运算 |
| `Alloca(IRType)` | 栈分配 |
| `Load(Value)` | 内存读取 |
| `Store(Value, Value)` | 内存写入 |
| `IntrinsicCall { intrinsic, args, ret_ty }` | 内建函数调用 |
| `ExternCall { name, args, ret_ty }` | 外部 C 函数调用 |
| `UserCall { name, args, ret_ty }` | 用户定义函数调用 |
| `Call { func, args, ret_ty }` | 间接函数调用 |

**Terminator (基本块终止符)**:

| 终止符 | 说明 |
|------|------|
| `Br(BlockId)` | 无条件跳转 |
| `CondBr(Value, BlockId, BlockId)` | 条件跳转 |
| `Switch(Value, BlockId, Vec<(i64, BlockId)>)` | Switch 跳转 |
| `Ret(Option<Value>)` | 返回 |
| `Unreachable` | 不可达 |

**Intrinsic (内建函数) 枚举**: 涵盖 70+ 种内建操作，包括 AI 调用 (`AiCall`, `AiStreamStart`)、上下文管理 (`ContextNew`, `ContextForget`)、I/O (`IoReadln`, `IoReadkey`)、HTTP、JSON、Actor、数据结构等。

**IRType 枚举**: `Void`, `I64`, `I32`, `F64`, `F32`, `Bool`, `String`, `Ptr`, `HttpResponse`, `JsonValue`, `AiResponse(Box<IRType>)`, `OpaqueStruct`。

---

### 4.10 ir_builder.rs — AST → IR 转换器

**路径**: [ir_builder.rs](file:///workspace/aial/aial-compiler/ir_builder.rs)

`IRBuilder` 是将类型化的 AST 转换为 IR Module 的核心组件。

```rust
pub struct IRBuilder {
    functions: Vec<IRFunction>,
    strings: Vec<String>,
    tool_registrations: Vec<ToolRegistration>,
    specializations: HashMap<...>,   // 泛型特化映射
    call_specializations: HashMap<...>, // 调用特化映射
    current_fn: Option<IRFnContext>,
    value_counter: u32,
    block_counter: u32,
}
```

**核心方法**:

| 方法 | 功能 |
|------|------|
| `build(program, type_env) -> IRModule` | 顶层入口，生成完整 IR Module |
| `collect_functions(items)` | 递归收集所有函数（含模块内的） |
| `declare_function(fn_def) -> IRFunction` | 声明函数签名 |
| `build_function(func, fn_def) -> IRFunction` | 构建函数体 IR |
| `emit_stmt(stmt)` | 发射语句 IR |
| `emit_expr(expr) -> Value` | 发射表达式 IR |
| `emit_block(block)` | 发射代码块 IR |

**控制流处理**:

| 方法 | 说明 |
|------|------|
| `emit_if_stmt(is)` | 生成 if-else 控制流 (then/else/merge 三块) |
| `emit_match_stmt(ms)` | 生成 match 控制流 (条件链 + 默认块) |
| `emit_for_stmt(fs)` | 生成 for 循环 IR (init/cond/body/inc/exit) |
| `emit_while_stmt(ws)` | 生成 while 循环 IR (cond/body/exit) |
| `emit_loop_stmt(ls)` | 生成 loop 循环 IR (body/exit) |

**内建函数识别**: `emit_expr` 中通过模式匹配识别 70+ 种内建函数调用（如 `println`、`ask`、`http::get`、`json::parse` 等），映射为对应的 `Intrinsic`。

**defer 处理**: 收集 `defer` 块，在函数返回前按 LIFO 顺序插入清理块。

**泛型单态化**: `build()` 方法中根据 `specializations` 映射生成特化后的函数副本。

**工具注册**: 识别 `#[tool]` 属性函数，生成 `ToolRegistration` 并序列化为 OpenAI 工具 JSON 格式。

---

### 4.11 ir_lower.rs — IR 降级与运行时函数注册

**路径**: [ir_lower.rs](file:///workspace/aial/aial-compiler/ir_lower.rs)

将高级 IR 指令降级为 `ExternCall`（外部 C 函数调用），同时构建 `RuntimeRegistry` 记录所有需要的运行时函数。

**核心结构**:
```rust
pub struct RuntimeRegistry {
    pub functions: Vec<RuntimeFunction>,
}

pub struct RuntimeFunction {
    pub name: String,      // e.g. "aial_rt_ai_call"
    pub params: Vec<IRType>,
    pub ret: IRType,
}
```

**降级映射**: 70+ 种 `Intrinsic` 变体 → 对应的 `aial_rt_*` C 函数名称 + 参数类型。

例如：
- `Intrinsic::AiCall` → `aial_rt_ai_call(model, ctx_id, prompt, temp, max_tokens, format) -> AiResponse`
- `Intrinsic::HttpGet` → `aial_rt_http_get(url) -> HttpResponse`
- `Intrinsic::JsonParse` → `aial_rt_json_parse(text) -> JsonValue`

---

### 4.12 interpreter.rs — IR 解释器

**路径**: [interpreter.rs](file:///workspace/aial/aial-compiler/interpreter.rs)

开发模式下的 IR 解释器，直接执行 IR 指令，无需 LLVM 编译。

**核心结构**:
```rust
struct EvalContext<'a> {
    values: HashMap<Value, i64>,          // IR 值 → 运行时值
    strings: &'a [String],                // 编译期字符串表
    tools: &'a [ToolRegistration],        // 工具注册表
    heap: HashMap<i64, i64>,              // 模拟堆内存
    string_store: HashMap<i64, String>,   // 运行时字符串存储
    next_addr: i64,                       // 地址分配器
    contexts: HashMap<i64, ContextState>, // AI 上下文状态
    tainted: HashSet<i64>,                // 污点跟踪
}
```

**ContextState**:
```rust
struct ContextState {
    id: i64,
    system_prompt: String,
    token_budget: i64,
    tokens_used: i64,
    hard_cap: bool,
    strategy: String,
    window_size: i64,
    cause_chain: Vec<(i64, String)>,   // 因果 DAG
    message_counter: i64,
    messages: Vec<String>,
}
```

**关键函数**:
- `interpret(module)`: 入口，查找 main 函数并执行
- `exec_func(ctx, func, args, module)`: 执行函数体
- `eval_instr(ctx, instr, ...)`: 执行单条 IR 指令
- `handle_runtime_call(ctx, name, args)`: 处理运行时函数调用

**特性**:
- **Mock 模式**: `AIAL_MOCK=1` 时返回模拟响应
- **真实 API 调用**: 支持 DeepSeek 和 OpenAI API
- **Token 预算管理**: 运行时强制 token 预算
- **因果 DAG**: 记录 AI 调用因果链，支持 `context::forget` 剪枝
- **污点跟踪**: `privacy::sensitive` 标记敏感数据，`println` 时警告
- **JSON 解析**: 使用 `serde_json` 完整支持 JSON 值操作
- **HTTP 客户端**: 使用 `reqwest` 支持 GET/POST 请求

---

### 4.13 llvm_backend.rs — LLVM 后端

**路径**: [llvm_backend.rs](file:///workspace/aial/aial-compiler/llvm_backend.rs)

零依赖 LLVM IR 文本生成器。直接生成 `.ll` 文本文件，无需链接 LLVM 库。

**核心函数**: `llvm_compile(module, reg, output_path) -> Result<(), String>`

**生成内容**:
1. **目标三元组**: `target triple = "x86_64-unknown-linux-gnu"`
2. **运行时函数声明**: 根据 `RuntimeRegistry` 生成 `declare` 语句
3. **全局字符串常量**: 将编译期字符串表生成为 `@.strN` 全局常量
4. **函数定义**: 每个 IR 函数转换为 LLVM `define` 块

**类型映射**:
| IR 类型 | LLVM 类型 |
|---------|----------|
| `IRType::Bool` | `i1` |
| `IRType::F64` | `double` |
| `IRType::Void` | `void` |
| 其他 | `i64` |

**指令转换**:
- `ConstInt(n)` → `add i64 0, n`
- `ConstFloat(f)` → `fadd double 0.0, f`
- `Alloca` → `alloca i64` + `ptrtoint`
- `Load` → `inttoptr` + `load`
- `Store` → `inttoptr` + `store`
- `Cmp` → `icmp`
- `BinOp` → `add/sub/mul/sdiv/srem/and/or`
- `ExternCall` → `call @aial_rt_*`
- `CondBr` → `br i1`

**特点**: main 函数返回 `i32`（遵循 C ABI），其他函数按 IR 类型返回。

---

### 4.14 capability.rs — 能力声明系统

**路径**: [capability.rs](file:///workspace/aial/aial-compiler/capability.rs)

实现了基于 `aial.toml` 配置文件的能力声明系统（法家理念）。

**配置结构**:
```rust
pub struct Config {
    pub capabilities: Option<Capabilities>,
    pub lints: Option<LintConfig>,
}

pub struct Capabilities {
    pub allow_network: Option<Vec<NetworkAccess>>,
    pub allow_filesystem: Option<Vec<FilesystemAccess>>,
}
```

**关键函数**:
- `load_config() -> Config`: 从 `aial.toml` 加载配置
- `check_provider_allowed(config, provider, model)`: 检查 AI provider/model 是否授权
- `check_filesystem_allowed(config, path, access)`: 检查文件系统访问权限
- `resolve_model(model_code) -> (provider, model_name)`: 解析模型代码

**默认模型映射**:
| 代码 | Provider | Model |
|------|----------|-------|
| 0 | deepseek | deepseek-v4-flash |
| 1 | deepseek | deepseek-v4-pro |
| 2 | openai | gpt-4o |
| 3 | openai | gpt-4o-mini |
| 4 | anthropic | claude-sonnet-4-6 |

**环境变量覆盖**: `AIAL_MODEL_<CODE>=provider:model`

---

### 4.15 key_manager.rs — API Key 管理

**路径**: [key_manager.rs](file:///workspace/aial/aial-compiler/key_manager.rs)

安全管理 API 密钥，遵循 "密钥不进入源代码" 原则。

**存储层级**:
1. 环境变量 `AIAL_KEY_<PROVIDER>`（优先）
2. `~/.aial/keys.json`（文件存储，0600 权限）

**关键函数**:
- `set_key(provider, key)`: 添加密钥到 `~/.aial/keys.json`
- `get_key(provider) -> String`: 获取密钥（环境变量优先）
- `list_keys() -> Vec<(provider, masked_key)>`: 列出所有密钥（掩码显示）
- `remove_key(provider)`: 删除密钥
- `first_provider() -> String`: 返回第一个已注册的 provider

---

### 4.16 philosophy.rs — 诊断风格引擎

**路径**: [philosophy.rs](file:///workspace/aial/aial-compiler/philosophy.rs)

提供三种哲学风格的编译错误诊断：

| 模式 | 风格 | 特点 |
|------|------|------|
| Tao (道家) | `--philosophy tao` | 温和、悖论式智慧，引用老子 |
| Legalist (法家) | `--philosophy legalist` | 严格、不妥协的法律风格，引用韩非子 |
| Medical (医家) | `--philosophy medical` | 诊断式，症状 → 处方 |

每种风格对不同类型的错误（未定义变量、类型错误、语法错误、能力未声明、预算耗尽等）提供不同的错误信息格式。

---

## 5. 模块详解 — aial-rt (运行时库)

**路径**: [lib.rs](file:///workspace/aial/aial-rt/src/lib.rs)  
**Crate 类型**: `staticlib` (静态库)  
**依赖**: `reqwest`, `serde`, `serde_json`, `tiny_http`, `rusqlite`, `libc`, `crossterm`

运行时库提供 80+ 个 `extern "C"` 函数，编译为 `libaial_rt.a`，由 clang 链接到最终二进制。

### 5.1 全局状态管理

运行时使用 `OnceLock<Mutex<T>>` 管理全局状态：

| 全局变量 | 类型 | 用途 |
|----------|------|------|
| `HEAP` | `HashMap<i64, i64>` | 堆内存 (地址 → 值) |
| `STRINGS` | `HashMap<i64, String>` | 字符串存储 (ID → 字符串) |
| `CONTEXTS` | `HashMap<i64, ContextState>` | AI 上下文 |
| `STREAM_TOKENS` | `HashMap<i64, (Arc<Mutex<Vec<String>>>, i64)>` | 流式 Token 缓冲 |
| `DB_CONNS` | `HashMap<i64, Arc<Mutex<Connection>>>` | SQLite 连接池 |
| `ACTOR_MAILBOXES` | `HashMap<i64, Arc<Mutex<Vec<String>>>>` | Actor 邮箱 |
| `MAPS` | `HashMap<i64, HashMap<String, String>>` | Map 数据结构 |
| `HEAPS` | `HashMap<i64, Vec<(String, i64)>>` | 优先队列 |
| `ARRAYS` | `HashMap<i64, Vec<String>>` | 动态数组 |
| `LINE_EDITORS` | `HashMap<i64, LineEditor>` | 行编辑器 |

---

### 5.2 AI 调用 (ask)

| C 函数 | 签名 | 功能 |
|--------|------|------|
| `aial_rt_ai_call` | `(model, ctx_id, prompt_idx, temperature, max_tokens, format) -> i64` | 同步 AI 调用，返回响应指针 |
| `aial_rt_ai_call_raw` | `(model, prompt_ptr, max_tokens) -> i64` | 裸 API 调用，无上下文/预算检查 |
| `aial_rt_ai_call_many` | `() -> i64` | 并行 AI 调用 (未完全实现) |
| `aial_rt_ai_call_race` | `() -> i64` | 竞速 AI 调用 (未完全实现) |
| `aial_rt_ai_stream_start` | `(model, ctx_id, prompt_idx, temperature, max_tokens, format, tools_json_idx) -> i64` | 启动流式 AI 调用，返回句柄 |
| `aial_rt_ai_stream_read` | `(handle) -> i64` | 读取流式 AI 的下一个 token |

**流式响应处理**:
- SSE (Server-Sent Events) 逐行解析
- 支持 `reasoning_content`（灰度显示）
- 支持 `tool_calls`（工具调用片段累积）
- 后台线程异步接收

**Mock 模式**: `AIAL_MOCK=1` 时返回模拟响应。

---

### 5.3 HTTP 客户端与服务端

| C 函数 | 功能 |
|--------|------|
| `aial_rt_http_get(url_ptr) -> i64` | HTTP GET 请求 |
| `aial_rt_http_post(url_ptr, body_ptr) -> i64` | HTTP POST 请求 |
| `aial_rt_http_post_json(url_ptr, val_ptr) -> i64` | HTTP POST JSON |
| `aial_rt_http_status(resp) -> i64` | 获取 HTTP 状态码 |
| `aial_rt_http_text(resp) -> i64` | 获取 HTTP 响应体文本 |
| `aial_rt_http_header_map() -> i64` | 创建 Header Map |
| `aial_rt_http_header_set(map, key, val) -> i64` | 设置 Header |
| `aial_rt_http_start(port) -> i64` | 启动 HTTP 服务器 |
| `aial_rt_http_listen(handle, timeout_ms) -> i64` | 监听 HTTP 请求 |
| `aial_rt_http_respond(req, body_ptr, ct_ptr)` | 响应 HTTP 请求 |
| `aial_rt_http_ok/json/html(req, body_ptr)` | 便捷响应方法 |
| `aial_rt_http_serve(req, path_ptr)` | 静态文件服务 |
| `aial_rt_http_body/method/url/path/query/header(req, ...)` | 请求信息提取 |

HTTP 响应布局: `[status, body_ptr, headers_ptr]`（偏移 0, 1, 2）

---

### 5.4 JSON 解析与操作

JSON 值在堆上的布局: `[type, aux, size/f64_bits, data_ptr, _]`

| 类型码 | JSON 类型 |
|--------|----------|
| 0 | null |
| 1 | bool |
| 2 | number |
| 3 | string |
| 4 | array |
| 5 | object |
| -1 | error |

| C 函数 | 功能 |
|--------|------|
| `aial_rt_json_parse(text_ptr) -> i64` | 解析 JSON 字符串 |
| `aial_rt_json_get(val_ptr, key_ptr) -> i64` | 对象字段访问 |
| `aial_rt_json_get_or(val_ptr, key_ptr, default_ptr) -> i64` | 带默认值的字段访问 |
| `aial_rt_json_type(val_ptr) -> i64` | 获取 JSON 类型 |
| `aial_rt_json_stringify(val_ptr) -> i64` | JSON → 字符串 |
| `aial_rt_json_value_to_string(val_ptr) -> i64` | JSON 值 → 字符串 |
| `aial_rt_json_to_int(val_ptr) -> i64` | JSON 值 → int |
| `aial_rt_json_to_float(val_ptr) -> f64` | JSON 值 → float |
| `aial_rt_json_array_len(val_ptr) -> i64` | 数组长度 |
| `aial_rt_json_array_get(val_ptr, idx) -> i64` | 数组索引访问 |

---

### 5.5 上下文管理 (Context)

| C 函数 | 功能 |
|--------|------|
| `aial_rt_ctx_new(prompt_ptr, budget, strategy, ws) -> i64` | 创建 AI 上下文 |
| `aial_rt_ctx_current() -> i64` | 获取当前上下文 |
| `aial_rt_ctx_budget(id) -> i64` | 查询剩余 token 预算 |
| `aial_rt_ctx_add_message(ctx_id, role_ptr, content_ptr) -> i64` | 添加消息到上下文 |
| `aial_rt_ctx_forget(ctx_id, msg_id)` | 因果剪枝 (GDPR) |
| `aial_rt_ctx_reflect(ctx_id) -> i64` | 自省/自我修正 |

**SQLite 记忆存储**:

| C 函数 | 功能 |
|--------|------|
| `aial_rt_ctx_open_memory(path_ptr) -> i64` | 打开 SQLite 数据库 |
| `aial_rt_ctx_save_message(db, session, role, content)` | 保存消息 |
| `aial_rt_ctx_load_messages(db, session, limit) -> i64` | 加载消息 (JSON) |
| `aial_rt_ctx_load_messages_since(db, session, ts) -> i64` | 按时间戳加载消息 |
| `aial_rt_ctx_close_memory(db)` | 关闭数据库 |
| `aial_rt_ctx_last_error() -> i64` | 获取最后错误 |

---

### 5.6 Actor 模型

| C 函数 | 功能 |
|--------|------|
| `aial_rt_actor_spawn() -> i64` | 创建 Actor (邮箱) |
| `aial_rt_actor_spawn_handler(fn_ptr, init_ptr) -> i64` | 创建 Actor + 线程处理函数 |
| `aial_rt_actor_send(pid, msg_ptr)` | 发送消息到 Actor |
| `aial_rt_actor_receive(pid) -> i64` | 阻塞接收消息 |
| `aial_rt_actor_try_receive(pid) -> i64` | 非阻塞接收 |
| `aial_rt_actor_recv_timeout(pid, timeout_ms) -> i64` | 超时接收 |
| `aial_rt_actor_error(pid) -> i64` | 获取 Actor 错误 |

`spawn_handler` 使用 `dlsym` 查找 AIAL 函数指针，在新线程中运行 Actor 处理函数。

---

### 5.7 I/O 与终端

**I/O 函数**:

| C 函数 | 功能 |
|--------|------|
| `aial_rt_println(text_ptr)` | 打印行 |
| `aial_rt_print(text_ptr)` | 打印 (无换行) |
| `aial_rt_io_readln() -> i64` | 读取一行 |
| `aial_rt_io_readln_timeout(ms) -> i64` | 超时读取一行 |
| `aial_rt_io_readkey() -> i64` | 读取单个按键 (crossterm) |
| `aial_rt_io_readkey_timeout(ms) -> i64` | 超时读取按键 |
| `aial_rt_io_read_multiline() -> i64` | 多行输入 |
| `aial_rt_io_raw_mode(enable)` | 切换终端 raw 模式 |

**行编辑器** (`LineEditor`): 提供带缓冲的行编辑功能，支持光标移动、历史记录、行重绘。

| C 函数 | 功能 |
|--------|------|
| `aial_rt_line_new(prompt_ptr) -> i64` | 创建行编辑器 |
| `aial_rt_line_read(handle) -> i64` | 读取一行（编辑模式） |
| `aial_rt_line_redraw(handle)` | 重绘行 |
| `aial_rt_line_end(handle)` | 结束编辑 |

**终端控制**:

| C 函数 | 功能 |
|--------|------|
| `aial_rt_term_clear()` | 清屏 |
| `aial_rt_term_height() -> i64` | 获取终端高度 |
| `aial_rt_term_scroll_region(top, bottom)` | 设置滚动区域 |
| `aial_rt_term_setup(rows)` | 终端初始化 (TUI) |
| `aial_rt_term_redraw(rows)` | 终端重绘 |
| `aial_rt_term_draw_text_clipped(row, col, width, text_idx)` | 裁剪绘制文本 |
| `aial_rt_term_cursor_row() -> i64` | 获取光标行 |
| `aial_rt_term_reset()` | 重置终端 |

---

### 5.8 数据结构 (Map, Heap, Array)

**Map (哈希表)**:

| C 函数 | 功能 |
|--------|------|
| `aial_rt_map_new() -> i64` | 创建 Map |
| `aial_rt_map_set(handle, key_idx, value_idx)` | 设置键值 |
| `aial_rt_map_get(handle, key_idx) -> i64` | 获取值 |
| `aial_rt_map_has(handle, key_idx) -> i64` | 检查键是否存在 |
| `aial_rt_map_remove(handle, key_idx)` | 删除键 |

**Heap (优先队列)**:

| C 函数 | 功能 |
|--------|------|
| `aial_rt_heap_new() -> i64` | 创建堆 |
| `aial_rt_heap_push(handle, value_idx, priority)` | 入堆 |
| `aial_rt_heap_pop(handle) -> i64` | 弹出最高优先级 |
| `aial_rt_heap_peek(handle) -> i64` | 查看最高优先级 |
| `aial_rt_heap_len(handle) -> i64` | 堆大小 |

**Array (动态数组)**:

| C 函数 | 功能 |
|--------|------|
| `aial_rt_array_new() -> i64` | 创建数组 |
| `aial_rt_array_push(handle, value_idx)` | 追加元素 |
| `aial_rt_array_sort(handle)` | 排序 |
| `aial_rt_array_get(handle, index) -> i64` | 按索引获取 |
| `aial_rt_array_len(handle) -> i64` | 数组长度 |

---

### 5.9 FFI (外部函数接口)

| C 函数 | 功能 |
|--------|------|
| `aial_rt_ffi_load(path_idx) -> i64` | 加载动态库 (dlopen) |
| `aial_rt_ffi_call(handle_id, fn_name_idx, a1..a6) -> i64` | 调用动态库函数 (dlsym) |
| `aial_rt_ffi_close(handle_id)` | 关闭动态库 (dlclose) |

---

### 5.10 其他函数

| C 函数 | 功能 |
|--------|------|
| `aial_rt_strcat/strlen/strslice/strchr/str_eq/starts_with` | 字符串操作 |
| `aial_rt_file_read/write/append/patch/list_dir` | 文件操作 |
| `aial_rt_html_escape(text_ptr) -> i64` | HTML 转义 |
| `aial_rt_process_run(cmd_idx) -> i64` | 执行 shell 命令 |
| `aial_rt_int_to_string(n) -> i64` | int → string |
| `aial_rt_string_to_int(s_idx) -> i64` | string → int |
| `aial_rt_args() -> i64` | 获取命令行参数 |
| `aial_rt_str_find(haystack_idx, needle_idx) -> i64` | 字符串查找 |
| `aial_rt_token_estimate(text_idx) -> i64` | Token 数量估算 |
| `aial_rt_time_now() -> i64` | 当前时间字符串 |
| `aial_rt_time_now_ms() -> i64` | 当前时间毫秒 |
| `aial_rt_time_sleep(ms)` | 线程休眠 |
| `aial_rt_string_register(idx, text_ptr)` | 注册编译期字符串 |
| `aial_rt_extract_ai_text/variant/reasoning/usage(resp) -> i64` | 提取 AI 响应字段 |
| `aial_rt_privacy_sensitive(val) -> i64` | 标记敏感数据 (污点跟踪) |
| `aial_rt_enum_create(name_ptr, variant_ptr) -> i64` | 创建枚举值 |
| `aial_rt_key_set/exists/delete` | API Key 运行时管理 |

---

## 6. 模块详解 — aial-vscode

**路径**: `aial-vscode/`

VS Code 扩展，提供 `.aal` 文件的语法高亮支持。基于 TextMate 语法定义。

---

## 7. 模块详解 — selfhost

**路径**: [selfhost/compiler.aal](file:///workspace/aial/selfhost/compiler.aal)

AIAL 语言编写的自举编译器。这是一个简化版的编译器，将 AIAL 源代码直接编译为 LLVM IR 文本。

**工作流程**:
1. 读取 `.aal` 源文件
2. 解析函数定义 (`fn` 关键字)
3. 识别 `println`/`print` 调用
4. 生成 LLVM IR 文本（`.ll` 文件）
5. 用户调用 `clang` 编译链接为二进制

**当前能力**: 解析简单的函数定义、字符串字面量、`println`/`print` 调用，生成对应的 LLVM IR。

---

## 8. 模块详解 — docs

| 文档 | 内容 |
|------|------|
| `GRAMMAR.md` | AIAL 语言形式语法规范 |
| `IR.md` | 中间表示 (IR) 的设计文档 |
| `PHILOSOPHY.md` | 语言设计哲学 |
| `STDLIB.md` | 标准库 API 文档 |
| `TYPE_SYSTEM.md` | 类型系统设计文档 |

---

## 9. 依赖关系

### 9.1 aial-compiler (Cargo.toml)

| 依赖 | 用途 |
|------|------|
| `clap` | CLI 参数解析 |
| `reqwest` (blocking) | HTTP 客户端 (AI API 调用) |
| `serde` / `serde_json` | JSON 序列化/反序列化 |
| `toml` | 解析 `aial.toml` 配置 |
| `chrono` | 密钥创建时间戳 |
| `rusqlite` (bundled) | SQLite 数据库 (上下文记忆) |
| `libc` | 系统调用 (termios, poll) |

### 9.2 aial-rt (Cargo.toml)

| 依赖 | 用途 |
|------|------|
| `reqwest` (blocking, rustls-tls, json) | HTTP 客户端 |
| `serde` / `serde_json` | JSON 处理 |
| `tiny_http` | HTTP 服务器 |
| `rusqlite` (bundled) | SQLite 数据库 |
| `libc` | 系统调用 (dlopen, poll, termios) |
| `crossterm` (events) | 终端按键事件处理 |

### 9.3 外部工具依赖

| 工具 | 用途 |
|------|------|
| `clang` | LLVM IR 编译与链接 |
| `libc-dev` | C 标准库头文件 |
| `Rust` (stable) | 编译编译器与运行时 |

---

## 10. 项目运行方式

### 10.1 环境准备

```bash
# Ubuntu/Debian
apt install build-essential clang

# macOS
xcode-select --install
```

### 10.2 构建

```bash
git clone https://github.com/gordonlu/aial.git
cd aial
bash build.sh
```

构建脚本 `build.sh` 依次执行：
1. `cd aial-compiler && cargo build --release` — 编译编译器
2. `cd aial-rt && cargo build --release` — 编译运行时库

### 10.3 运行模式

**解释执行 (开发模式)**:
```bash
aial run examples/01_hello.aal
```

**LLVM AOT 编译 (生产模式)**:
```bash
aial build examples/01_hello.aal
clang aial_output.ll -L aial-rt/target/release -laial_rt -lm -lpthread -ldl -o binary
./binary
```

### 10.4 API Key 配置

```bash
# 添加密钥
aial key add --provider deepseek --key sk-xxx

# 或使用环境变量
export AIAL_KEY_DEEPSEEK=sk-xxx

# Mock 模式 (无需 API Key)
AIAL_MOCK=1 aial run examples/01_hello.aal
```

### 10.5 能力声明 (aial.toml)

```toml
[capabilities]
allow_network = [
    { provider = "deepseek", models = ["deepseek-v4-flash"] }
]
allow_filesystem = [
    { path = "./data", access = "read" }
]

[lints]
unused_match_variable = "warn"
```

### 10.6 自举编译

```bash
cd selfhost
aial build compiler.aal
clang aial_output.ll -L ../aial-rt/target/release -laial_rt -lm -lpthread -ldl -rdynamic -o aialc
./aialc hello.aal  # 自举编译器编译 hello.aal
```

### 10.7 诊断风格切换

```bash
aial run --philosophy tao file.aal     # 道家风格
aial run --philosophy legalist file.aal # 法家风格
aial run --philosophy medical file.aal  # 医家风格
```

---

## 11. 示例代码与测试

### 11.1 示例程序 (examples/)

| 文件 | 说明 |
|------|------|
| `01_hello.aal` | Hello World + ask 示例 |
| `02_parallel.aal` | 并行 AI 调用 |
| `03_budget.aal` | Token 预算管理 |
| `04_loop.aal` | 循环控制流 |
| `05_match.aal` | Match 模式匹配 |
| `06_webui.aal` | HTTP 服务器 Web UI |
| `07_json.aal` | JSON 解析 |
| `08_chat.aal` | 对话程序 |
| `aial_lexer.aal` | 用 AIAL 编写的词法分析器 |
| `aial_parser.aal` | 用 AIAL 编写的语法分析器 |

### 11.2 集成测试 (tests/integration.rs)

测试编译器流水线的关键功能：
- 基本类型系统测试
- 泛型测试
- Match 穷尽性测试
- 模块系统测试

---

> 本文档由代码分析自动生成，基于 AIAL v0.5 代码库 (commit 82559de, 2026-05-15)。
