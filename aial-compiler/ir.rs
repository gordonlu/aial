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
    HttpResponse,
    JsonValue,
    OpaqueStruct,
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
    UserCall {                 // 用户自定义函数调用（带名称）
        name: String,
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
    AiStreamStart,  // ask(stream=true) → stream handle
    // 上下文
    ContextNew,
    ContextCurrent,
    ContextBudget,
    ContextAddMessage,  // context::add_message(ctx, role, content) -> ctx
    // 提取 AiResponse 字段
    ExtractAiText,
    ExtractAiVariant,
    ExtractAiUsage,
    ExtractAiReasoning,
    // 工具
    ToolDispatch,
    CapCheck,
    // Actor
    ActorSpawn,        // actor::spawn() -> pid
    ActorSpawnHandler,  // actor::spawn_handler(fn_name, init_msg) -> pid (threaded)
    ActorSend,          // actor::send(pid, msg) -> void
    ActorReceive,       // actor::recv(pid) -> string (blocking)
    ActorTryReceive,    // actor::try_recv(pid) -> string ("" if empty)
    ActorRecvTimeout,   // actor::recv_timeout(pid, timeout_ms) -> string
    ActorError,         // actor::error(pid) -> string
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
    FilePatch,
    // Enum construction/destruction
    EnumCreate,  // EnumCreate(type, variant, args...) → ptr
    // HTTP (bootstrapping)
    HttpGet,     // http::get(url) → HttpResponse handle
    HttpStatus,  // http::status(resp) → int
    HttpText,    // http::text(resp) → string
    // JSON (bootstrapping)
    JsonParse,   // json::parse(text) → JsonValue handle
    JsonGet,     // json::get(val, key) → JsonValue handle
    JsonGetOr,   // json::get_or(val, key, default) → JsonValue handle
    JsonType,    // json::type_of(val) → int (0=null,1=bool,2=number,3=string,4=array,5=object)
    // JSON (more)
    JsonToString,      // json::stringify(val) → string
    JsonValueToString, // json::to_string(val) → string
    JsonToInt,         // json::to_int(val) → int
    JsonToFloat,   // json::to_float(val) → float
    JsonArrayLen,  // json::array_len(val) → int
    JsonArrayGet,  // json::array_get(val, idx) → JsonValue
    // HTTP (more)
    HttpPost,          // http::post(url, body) → HttpResponse
    HttpPostJson,      // http::post_json(url, json_val) → HttpResponse
    HttpHeaderMap,     // http::header_map() → HeaderMap handle
    HttpHeaderSet,     // http::header_set(map, key, val) → HeaderMap
    HttpStart,         // http::start(port) → ServerHandle
    HttpListen,        // http::listen(handle, timeout_ms) → Request
    HttpRespond,       // http::respond(req, body, content_type) → void
    HttpRequestBody,   // http::body(req) → string
    HttpMethod,        // http::method(req) → string
    HttpPath,          // http::path(req) → string
    HttpQuery,         // http::query(req, key) → string
    HttpHeader,        // http::header(req, key) → string
    HttpUrl,           // http::url(req) → string
    HttpStatusText,    // http::status_text(code) → string
    HttpOk,            // http::ok(req, body) → void
    HttpJson,          // http::json(req, body) → void
    HttpHtml,          // http::html(req, body) → void
    HttpServe,         // http::serve(req, path) → void
    // HTML
    HtmlEscape,   // html::escape(text) → string
    // AI streaming
    AiStreamRead,  // ask::read_token(handle) → string
    AiCallRaw,     // ask_raw(model, prompt, max_tokens) → string (no capability check)
    // I/O
    IoReadln,        // io::readln() → string
    IoReadlnTimeout, // io::readln_timeout(ms) → string
    IoReadkey,         // io::readkey() → string (single char, raw mode aware)
    IoReadkeyTimeout,   // io::readkey_timeout(ms) → string ("" if timeout)
    IoReadMultiline,    // io::read_multiline() → string
    IoRawMode,          // io::raw_mode(bool) → void
    // Print (without newline)
    Print,           // print(text) → void
    // Memory (SQLite-backed context memory)
    CtxOpenMemory,       // ctx::open_memory(path) → db handle
    CtxSaveMessage,      // ctx::save_message(db, session, role, content)
    CtxLoadMessages,     // ctx::load_messages(db, session, limit) → JSON string
    CtxLoadMessagesSince,// ctx::load_messages_since(db, session, timestamp) → JSON string
    CtxCloseMemory,      // ctx::close_memory(db)
    CtxLastError,        // ctx::last_error() → string
    // Time
    TimeSleep,       // time::sleep(ms)
    TimeNow,         // time::now() -> string
    TimeNowMs,       // time::now_ms() -> int (millisecond timestamp)
    TermDrawClipped,  // term::draw_text_clipped(row, col, width, text) -> void
    TermCursorRow,    // term::cursor_row() -> int
    // Self-hosting essentials
    ProcessRun,       // process::run(cmd) -> string
    IntToString,      // int_to_string(n) -> string
    StringToInt,      // string_to_int(s) -> int
    Args,             // args() -> string (arg list, newline-separated)
    StrFind,          // str_find(haystack, needle) -> int (index or -1)
    FileListDir,      // file::list_dir(path) -> string (paths, newline-separated)
    TermClear,       // term::clear() -> void
    TermHeight,      // term::height() -> int
    TermSetup,       // term::setup(rows) -> void (sets scroll region + draws bottom)
    TermRedraw,      // term::redraw(rows) -> void (redraw bottom area only)
    TermScroll,      // term::scroll_region(top, bottom) -> void (deprecated, use setup)
    TermReset,       // term::reset() -> void (deprecated)
    TermCursorGoto,  // term::cursor_goto(row, col) -> void (deprecated)
    // Line editor
    LineNew,         // line::new(prompt) -> handle
    LineRead,        // line::read(handle) -> string
    LineRedraw,      // line::redraw(handle) -> void (force bottom redraw)
    LineEnd,         // line::end(handle) -> void
    // FFI
    FfiLoad,         // ffi::load(path) → lib handle
    FfiCall,         // ffi::call(handle, fn_name, args...) → result
    FfiClose,        // ffi::close(handle)
    // Map (hash table)
    MapNew,          // map::new() → handle
    MapSet,          // map::set(handle, key, value) → void
    MapGet,          // map::get(handle, key) → string
    MapHas,          // map::has(handle, key) → bool
    MapRemove,       // map::remove(handle, key) → void
    // Key management (embedded in runtime)
    KeySet,          // key::set(provider, key) → bool
    KeyExists,       // key::exists(provider) → bool
    KeyDelete,       // key::delete(provider) → bool
    // Token estimation
    TokenEstimate,   // token_estimate(text) → int
    // Priority queue (heap)
    HeapNew,          // heap::new() → handle
    HeapPush,         // heap::push(handle, value, priority) → void
    HeapPop,          // heap::pop(handle) → string
    HeapPeek,         // heap::peek(handle) → string
    HeapLen,          // heap::len(handle) → int
    // Array
    ArrayNew,         // array::new() → handle
    ArrayPush,        // array::push(handle, value) → void
    ArraySort,        // array::sort(handle) → void
    ArrayGet,         // array::get(handle, index) → string
    ArrayLen,         // array::len(handle) → int
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
