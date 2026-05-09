# AIAL 类型系统

## 基础类型
`int float bool string null int8…uint64 float32 float64`

## 结构化类型
- `struct`：可包含字段，支持泛型、默认值。
- `enum`：每个变体可带载荷。
- `trait`：定义方法签名，通过 `impl` 实现。
- 函数类型：`fn(T1, …, Tn) -> R`。
- 可选类型：`T?` 语法糖，对应 `Option<T>`。
- 联合类型：`T1 | T2 | ...`。

## 特殊类型
- `dynamic`：动态类型，从 AI 返回的自然语言初始为 `dynamic`，需通过模式匹配或 `as` 显式转换。
- `Model`：AI 模型枚举，如 `Model::GPT_4O`。
- `Context`：上下文句柄，由标准库 `context` 模块创建和管理。
- `AiResponse<T>`：`ask` 表达式的返回类型，有四个变体：
  - `Success { text: T, reasoning: string?, usage: Usage }`
  - `Degraded { text: T, reason: DegradeReason, usage: Usage }`
  - `Refused { reason: string }`
  - `Error { error: AiError }`
- `Usage`：`{ prompt_tokens: int, completion_tokens: int, total_tokens: int }`

## 类型检查
采用双向类型检查（局部推导 + 已知期望检查）。  
- 对 `ask` 表达式，检查选项参数类型：
  - `model` 必须为 `Model`
  - `context` 必须为 `Context`
  - `prompt` 必须为 `string`
  - `temperature`/`top_p` 必须为 `float`
  - `max_tokens` 必须为 `int`
- 模式匹配 `AiResponse` 必须穷尽所有变体（编译器强制）。

## Tool trait
`#[tool]` 注解的函数自动实现 `Tool` trait，要求参数和返回类型实现 `Serialize`/`Deserialize`。编译器验证所需能力是否已在 `aial.toml` 的 `[capabilities]` 声明。

## 类型推导
- 局部变量从初始化表达式推导类型，无需显式注解。
- 泛型函数每次调用生成新鲜类型变量并合一。
- 递归函数必须标注返回类型。
- 管道 `e1 |> e2` 将 `e2` 视为 `fn(e1的类型) -> ?` 并推导。
