// ast.rs - AAL 语言抽象语法树定义

use crate::token::Span;

// ============================================================
// 顶层结构
// ============================================================

/// 整个程序
#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<TopLevelItem>,
    pub main_fn: Option<FnDef>,
}

/// 顶层项
#[derive(Debug, Clone)]
pub enum TopLevelItem {
    Use(UseStmt),
    Test(FnDef),
    FnDef(FnDef),
    TypeDef(TypeAlias),
    StructDef(StructDef),
    EnumDef(EnumDef),
    TraitDef(TraitDef),
    ImplBlock(ImplBlock),
}

/// use 语句
#[derive(Debug, Clone)]
pub struct UseStmt {
    pub span: Span,
    pub path: Path,
}

/// 路径 aial::tool::get_time
#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub segments: Vec<Ident>,
}

/// 标识符
#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

// ============================================================
// 类型
// ============================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Var(u32),                      // 类型变量（类型推导用）
    Base(BaseType),
    Dynamic,
    Path(Path, Option<Vec<Type>>), // 路径 + 可选泛型参数
    Optional(Box<Type>),           // T?
    Union(Vec<Type>),              // T | U | V
    Fn(Vec<Type>, Box<Type>),      // fn(...) -> R
    Array(Box<Type>, u64),         // [T; N]
    Slice(Box<Type>),              // [T]
}

#[derive(Debug, Clone, PartialEq)]
pub enum BaseType {
    Int, Int8, Int16, Int32, Int64,
    Uint8, Uint16, Uint32, Uint64,
    Float32, Float64, Float,
    Bool, String, Null,
    ApiKey, // 不透明类型，不可打印/序列化/转换
}

// ============================================================
// 函数
// ============================================================

#[derive(Debug, Clone)]
pub struct FnDef {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub name: Ident,
    pub generics: Option<Vec<Ident>>,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub span: Span,
    pub mutable: bool,
    pub name: Ident,
    pub ty: Type,
    pub default: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub span: Span,
    pub stmts: Vec<Stmt>,
    pub trailing_expr: Option<Box<Expr>>,
    pub parallel: bool,   // #[parallel] annotation — asks inside may be merged
}

// ============================================================
// 语句
// ============================================================

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(LetStmt),
    Assign(AssignStmt),
    Return(Option<Expr>, Span),
    Expression(Expr),
    If(IfStmt),
    Match(MatchStmt),
    For(ForStmt),
    While(WhileStmt),
    Loop(LoopStmt),
    Break(Span),
    Continue(Span),
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub span: Span,
    pub mutable: bool,
    pub name: Ident,
    pub ty: Option<Type>,
    pub init: Expr,
}

#[derive(Debug, Clone)]
pub struct AssignStmt {
    pub span: Span,
    pub target: LValue,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub enum LValue {
    Variable(Ident),
    Field(Box<LValue>, Ident),
    Index(Box<LValue>, Box<Expr>),
    Deref(Box<LValue>),
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub span: Span,
    pub cond: Expr,
    pub then_block: Block,
    pub else_branch: Option<Box<ElseBranch>>,
}

#[derive(Debug, Clone)]
pub enum ElseBranch {
    If(Box<IfStmt>),
    Block(Block),
}

#[derive(Debug, Clone)]
pub struct MatchStmt {
    pub span: Span,
    pub scrutinee: Expr,
    pub arms: Vec<MatchArm>,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub span: Span,
    pub pattern: Pattern,
    pub body: MatchBody,
}

#[derive(Debug, Clone)]
pub enum MatchBody {
    Block(Block),
    Expr(Expr), // 单表达式（逗号结尾）
}

#[derive(Debug, Clone)]
pub struct ForStmt {
    pub span: Span,
    pub variable: Ident,
    pub iterator: Expr,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub span: Span,
    pub cond: Expr,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct LoopStmt {
    pub span: Span,
    pub body: Block,
}

// ============================================================
// 表达式
// ============================================================

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    // 字面量
    IntLiteral(u64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    NullLiteral,

    // 变量/self
    Variable(Ident),
    SelfExpr,

    // 一元
    Unary(UnOp, Box<Expr>),
    // 二元
    Binary(BinOp, Box<Expr>, Box<Expr>),
    // 管道 e1 |> e2
    Pipe(Box<Expr>, Box<Expr>),

    // 函数调用 fn(args) 或 expr(args)，支持命名参数
    Call(Box<Expr>, Vec<Expr>, Vec<AskOption>),
    // 方法调用 expr.method::<generic>(args)
    MethodCall {
        receiver: Box<Expr>,
        method: Ident,
        generic_args: Option<Vec<Type>>,
        args: Vec<Expr>,
    },
    // 索引 expr[index]
    Index(Box<Expr>, Box<Expr>),

    // 字段访问 expr.field
    FieldAccess { receiver: Box<Expr>, field: Ident },

    // 结构体字面量 StructName { field: val, ... }
    StructLiteral {
        struct_name: Path,
        fields: Vec<(Ident, Expr)>,
    },

    // if 表达式 if cond { then } else { else_ }
    IfExpr(Box<Expr>, Block, Box<Expr>),
    // match 表达式
    MatchExpr(Box<Expr>, Vec<MatchArm>),
    // 块表达式 { ... }
    BlockExpr(Block),

    // ask 调用
    Ask(Vec<AskOption>),
    AskMany(Vec<Vec<AskOption>>),
    AskRace(Vec<Vec<AskOption>>),

    // receive 表达式
    Receive,

    // 路径引用 (如 aial::tool::get_time)
    Path(Path),
}

#[derive(Debug, Clone)]
pub enum UnOp {
    Neg, // -
    Not, // !
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add, Sub, Mul, Div, Rem,
    Eq, Ne, Lt, Gt, Le, Ge,
    And, Or,
}

// ============================================================
// ask 专用
// ============================================================

#[derive(Debug, Clone)]
pub struct AskOption {
    pub name: Ident,
    pub value: Expr,
}

// ============================================================
// 模式
// ============================================================

#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard(Ident),                         // _
    Variable(Ident),                         // x
    Literal(Expr),                           // 42, true, "hello"
    Constructor(Path, Vec<Pattern>),        // Some(x), Error(code, msg)
    Or(Vec<Pattern>),                        // p1 | p2 | p3
    As(Box<Pattern>, Ident),                // p as x
}

// ============================================================
// 注解
// ============================================================

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: Ident,
    pub args: Vec<AttrArg>,
}

#[derive(Debug, Clone)]
pub enum AttrArg {
    Named { name: Ident, value: AttrValue },
    Unnamed(AttrValue),
}

#[derive(Debug, Clone)]
pub enum AttrValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Ident(String),
    Array(Vec<AttrValue>),
}

// ============================================================
// 其他类型定义
// ============================================================

#[derive(Debug, Clone)]
pub struct TypeAlias {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub name: Ident,
    pub generics: Option<Vec<Ident>>,
    pub ty: Type,
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub name: Ident,
    pub generics: Option<Vec<Ident>>,
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub span: Span,
    pub name: Ident,
    pub ty: Type,
    pub default: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct EnumDef {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub name: Ident,
    pub generics: Option<Vec<Ident>>,
    pub variants: Vec<VariantDef>,
}

#[derive(Debug, Clone)]
pub struct VariantDef {
    pub span: Span,
    pub name: Ident,
    pub payload: Option<Vec<Type>>,
    pub discriminant: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct TraitDef {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub name: Ident,
    pub generics: Option<Vec<Ident>>,
    pub methods: Vec<MethodSig>,
}

#[derive(Debug, Clone)]
pub struct MethodSig {
    pub span: Span,
    pub name: Ident,
    pub generics: Option<Vec<Ident>>,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct ImplBlock {
    pub span: Span,
    pub generics: Option<Vec<Ident>>,
    pub trait_name: Option<Path>,
    pub target_type: Type,
    pub methods: Vec<FnDef>,
}
