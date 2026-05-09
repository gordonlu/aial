# AIAL 中间表示 (AIAL-IR)

## 设计理念
AIAL-IR 是承载高级 Agent 语义的 SSA 形式中间表示，分为三层：
- **理**：控制流、调用、权限检查（`br`, `call`, `ret` 等）
- **数**：资源管理、预算查询（`ctx_budget` 等）
- **象**：AI 交互、文本提取（`ai_call`, `extract_ai_text` 等）

## 基本结构
- `IRModule`：包含多个 `IRFunction` + 字符串常量表 + 工具注册表。
- `IRFunction`：由 `BasicBlock` 列表组成，入口块由 `entry` 指定。
- `BasicBlock`：包含指令序列和一个终止符（`Terminator`）。
- 所有临时值通过 `Value(u32)` 编号，SSA 形式（每条指令产生一个新的 `Value`）。

## 核心指令

| 类别 | 指令 | 说明 |
|------|------|------|
| 常量 | `ConstInt(i64)` `ConstFloat(f64)` `ConstString(String)` `ConstBool(bool)` `ConstNull` | 产生常量值 |
| 运算 | `BinOp(BinOp, Value, Value)` `UnOp(UnOp, Value)` `Cmp(BinOp, Value, Value)` | 算术/逻辑/比较 |
| 内存 | `Alloca(IRType)` `Load(Value)` `Store(Value, Value)` | 局部变量 |
| 聚合 | `ExtractValue { aggregate, index }` `InsertValue { aggregate, element, index }` | 结构体操作 |
| 控制流 | `Br(BlockId)` `CondBr(Value, BlockId, BlockId)` `Switch(Value, default, cases)` `Ret(Option<Value>)` `Unreachable` | 块间跳转 |
| 调用 | `Call { func, args, ret_ty }` `ExternCall { name, args, ret_ty }` | 普通调用和外部运行时调用 |
| 内置 | `IntrinsicCall { intrinsic, args, ret_ty }` | 高级 AI 操作（降低前使用） |

## 内置函数 (Intrinsic)
在 IR 降低阶段，以下 `IntrinsicCall` 被替换为 `ExternCall`，调用运行时实现：

| Intrinsic | 运行时函数名 |
|-----------|-------------|
| `AiCall` | `aial_rt_ai_call` |
| `AiCallMany` | `aial_rt_ai_call_many` |
| `ContextNew` | `aial_rt_ctx_new` |
| `ContextCurrent` | `aial_rt_ctx_current` |
| `ExtractAiText` | `aial_rt_extract_ai_text` |
| `ExtractAiVariant` | `aial_rt_extract_ai_variant` |
| `Println` | `aial_rt_println` |
| ... | ... |

## 结束语
AIAL-IR 保证高级语义不被过早丢失，便于优化和静态分析。降低后的 IR 仅包含 `ExternCall`，可直接映射为 Cranelift CLIF 或 LLVM IR。
