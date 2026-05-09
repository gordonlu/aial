// ir.rs - AAL 中间表示定义

use crate::ast::{BinOp, UnOp};

// ============================================================
// 基本类型
// ============================================================

/// IR 中的值引用
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Value(pub u32);

/// 基本块标识
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub u32);

// ============================================================
// IR 类型
// ============================================================

#[derive(Debug, Clone, PartialEq)]
pub enum IRType {
    I32, I64,
    F32, F64,
    Bool,
    Ptr,                     // 不透明指针（如 Context*）
    Void,
    Array(Box<IRType>, u64),
    Struct(Vec<IRType>),
    // 对应高级类型
    String,
    Context,
    Model,
    AiResponse(Box<IRType>),
    AiManyResponse(Box<IRType>),
    AiRaceResponse(Box<IRType>),
}

// ============================================================
// 指令
// ============================================================

#[derive(Debug, Clone)]
pub enum Instr {
    // 常量
    ConstInt(i64),
    ConstFloat(f64),
    ConstString(String),
    ConstBool(bool),
    ConstNull,

    // 二元运算
    BinOp(BinOp, Value, Value),
    // 一元运算
    UnOp(UnOp, Value),
    // 比较（结果 i1）
    Cmp(BinOp, Value, Value),

    // 内存
    Alloca(IRType),
    Load(Value),
    Store(Value, Value),

    // 聚合操作
    ExtractValue {
        aggregate: Value,
        index: u32,
    },
    InsertValue {
        aggregate: Value,
        element: Value,
        index: u32,
    },

    // 控制流相关
    Phi(Vec<(Value, BlockId)>),
    Br(BlockId),
    CondBr(Value, BlockId, BlockId),
    Switch(Value, BlockId, Vec<(i64, BlockId)>), // 默认块 + 值-块对
    Ret(Option<Value>),
    Unreachable,

    // 函数调用
    Call {
        func: Value,           // 函数指针（通常为全局函数名映射）
        args: Vec<Value>,
        ret_ty: IRType,
    },
    // 内置函数调用（用于 ask, context 等）
    IntrinsicCall {
        intrinsic: Intrinsic,
        args: Vec<Value>,
        ret_ty: IRType,
    },
    
    ExternCall {
        name: String,
        args: Vec<Value>,
        ret_ty: IRType,
    },
}

// ============================================================
// AI 内置函数
// ============================================================

#[derive(Debug, Clone)]
pub enum Intrinsic {
    // AI 调用
    AiCall,
    AiCallMany,
    AiCallRace,
    // 上下文
    ContextNew,
    ContextCurrent,
    ContextBudget,
    // 提取 AiResponse 字段
    ExtractAiText,
    ExtractAiVariant,
    ExtractAiUsage,
    ExtractAiReasoning,
    // 工具
    ToolDispatch,
    CapCheck,
    // Actor
    ActorSpawn,
    ActorSend,
    ActorReceive,
    // 打印
    Println,
    // 隐私
    PrivacySensitive,
    // Context management
    ContextForget,
    ContextReflect,
    // String ops (bootstrapping)
    StrLen,      // strlen(s) → int
    StrConcat,   // strcat(a, b) → string
    StrSlice,
    StrChr,
    StrEq,       // str_eq(a, b) → bool — content comparison
    StartsWith,  // starts_with(s, prefix) → bool
    // File I/O (bootstrapping)
    FileRead,
    FileWrite,
    FileAppend,  // file::append(path, content) → void
    FilePatch,   // file::patch(path, replace=(old, new)) → void
}

// ============================================================
// 基本块与函数
// ============================================================

#[derive(Debug)]
pub struct BasicBlock {
    pub id: BlockId,
    pub instrs: Vec<(Instr, Option<Value>)>,  // instruction + optional result Value
    pub terminator: Option<Terminator>,
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Br(BlockId),
    CondBr(Value, BlockId, BlockId),
    Switch(Value, BlockId, Vec<(i64, BlockId)>),
    Ret(Option<Value>),
    Unreachable,
}

#[derive(Debug)]
pub struct IRFunction {
    pub name: String,
    pub params: Vec<(Value, IRType)>,
    pub return_type: IRType,
    pub blocks: Vec<BasicBlock>,
    /// 函数入口块
    pub entry: BlockId,
    /// 值到类型的映射
    pub value_types: Vec<(Value, IRType)>,
}

/// 整个程序的 IR 模块
#[derive(Debug)]
pub struct IRModule {
    pub functions: Vec<IRFunction>,
    /// 字符串常量表
    pub strings: Vec<String>,
    /// 全局工具注册表
    pub tool_registrations: Vec<ToolRegistration>,
}

#[derive(Debug, Clone)]
pub struct ToolRegistration {
    pub name: String,
    pub description: String,
    pub risk_level: String,
    pub required_caps: Vec<String>,
    pub fn_ptr: Value,
    pub idempotent: bool,   // true = safe to retry on failure
    pub version: String,    // signature hash for compatibility check
    pub fallback: Option<String>, // fallback tool name on failure
}
