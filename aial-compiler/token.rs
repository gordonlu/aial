// token.rs - AAL 语言的 Token 定义

/// 源代码位置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,   // 字节偏移
    pub end: usize,
    pub line: usize,
    pub col: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, col: usize) -> Self {
        Span { start, end, line, col }
    }
}

/// Token 类型
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // === 关键字 ===
    Fn,
    Let,
    Mut,
    Return,
    If,
    Else,
    Match,
    As,
    Type,
    Module,
    Struct,
    Enum,
    Trait,
    Impl,
    Ask,
    Context,
    Tool,
    Use,
    Test,
    Receive,
    SelfKw,
    For,
    In,
    While,
    Loop,
    Break,
    Continue,
    Defer,
    True,
    False,
    Null,

    // === 字面量 ===
    Int(u64),          // 整数字面量，数值
    Float(f64),        // 浮点字面量，数值
    String(String),    // 字符串字面量，内容（已处理转义）

    // === 标识符 ===
    Ident(String),     // 标识符名字（区分大小写）

    // === 符号 ===
    Plus,              // +
    Minus,             // -
    Star,              // *
    Slash,             // /
    Percent,           // %
    EqEq,              // ==
    NotEq,             // !=
    Lt,                // <
    Gt,                // >
    LtEq,              // <=
    GtEq,              // >=
    AndAnd,            // &&
    OrOr,              // ||
    Not,               // !
    Pipe,              // |
    PipeGt,            // |>
    Dot,               // .
    DotDot,            // ..
    Colon,             // :
    ColonColon,        // ::
    Semicolon,         // ;
    Comma,             // ,
    Lparen,            // (
    Rparen,            // )
    Lbrace,            // {
    Rbrace,            // }
    Lbracket,          // [
    Rbracket,          // ]
    Arrow,             // ->
    FatArrow,          // =>
    Assign,            // =
    AtSign,            // @ (预留)
    Hash,              // # (单独使用时报错)
    AttrStart,         // #[ (注解开始)

    Eof,               // 文件结束
    Error(String),     // 词法错误（携带消息）
}

/// 单个 Token
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }
}

// 辅助函数：将关键字字符串映射为 TokenKind
pub fn lookup_keyword(word: &str) -> Option<TokenKind> {
    match word {
        "fn" => Some(TokenKind::Fn),
        "let" => Some(TokenKind::Let),
        "mut" => Some(TokenKind::Mut),
        "return" => Some(TokenKind::Return),
        "if" => Some(TokenKind::If),
        "else" => Some(TokenKind::Else),
        "match" => Some(TokenKind::Match),
        "as" => Some(TokenKind::As),
        "type" => Some(TokenKind::Type),
        "module" => Some(TokenKind::Module),
        "struct" => Some(TokenKind::Struct),
        "enum" => Some(TokenKind::Enum),
        "trait" => Some(TokenKind::Trait),
        "impl" => Some(TokenKind::Impl),
        "ask" => Some(TokenKind::Ask),
        "use" => Some(TokenKind::Use),
        "test" => Some(TokenKind::Test),
        "receive" => Some(TokenKind::Receive),
        "self" => Some(TokenKind::SelfKw),
        "for" => Some(TokenKind::For),
        "in" => Some(TokenKind::In),
        "while" => Some(TokenKind::While),
        "loop" => Some(TokenKind::Loop),
        "break" => Some(TokenKind::Break),
        "continue" => Some(TokenKind::Continue),
        "defer" => Some(TokenKind::Defer),
        "true" => Some(TokenKind::True),
        "false" => Some(TokenKind::False),
        "null" => Some(TokenKind::Null),
        _ => None,
    }
}
