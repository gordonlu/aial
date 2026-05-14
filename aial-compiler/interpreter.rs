// interpreter.rs — AAL IR interpreter
// Replaces the deprecated cranelift-simplejit backend.
// Will be superseded by cranelift-jit or LLVM once the language stabilizes.

use crate::ir::*;
use std::collections::HashMap;
use std::path::Path;

pub fn interpret(module: &IRModule) -> Result<(), String> {
    let main_fn = module
        .functions
        .iter()
        .find(|f| f.name == "main")
        .ok_or_else(|| "no main function found".to_string())?;

    let mut ctx = EvalContext::new(&module.strings, &module.tool_registrations);
    exec_func(&mut ctx, main_fn, &[], module)?;
    Ok(())
}

struct ContextState {
    id: i64,
    system_prompt: String,
    token_budget: i64,
    tokens_used: i64,
    hard_cap: bool,
    strategy: String,
    window_size: i64,
    cause_chain: Vec<(i64, String)>,  // #9: causal DAG entries (id, description)
    message_counter: i64,             // #12: message ID counter
    messages: Vec<String>,            // context::add_message history
}

struct EvalContext<'a> {
    values: HashMap<Value, i64>,
    strings: &'a [String],
    tools: &'a [ToolRegistration],
    heap: HashMap<i64, i64>,
    string_store: HashMap<i64, String>,
    next_addr: i64,
    contexts: HashMap<i64, ContextState>,
    next_ctx_id: i64,
    tainted: std::collections::HashSet<i64>,  // taint-tracking for privacy::sensitive
}

impl<'a> EvalContext<'a> {
    fn new(strings: &'a [String], tools: &'a [ToolRegistration]) -> Self {
        EvalContext {
            values: HashMap::new(),
            strings,
            tools,
            heap: HashMap::new(),
            string_store: HashMap::new(),
            next_addr: 1,
            contexts: HashMap::new(),
            next_ctx_id: 1,
            tainted: std::collections::HashSet::new(),
        }
    }

    fn alloc(&mut self) -> i64 {
        let addr = self.next_addr;
        self.next_addr += 1;
        addr
    }

    fn alloc_block(&mut self, size: usize) -> i64 {
        let addr = self.next_addr;
        self.next_addr += size as i64;
        addr
    }
}

fn exec_func(
    ctx: &mut EvalContext,
    func: &IRFunction,
    args: &[i64],
    module: &IRModule,
) -> Result<Option<i64>, String> {
    for (i, (v, _)) in func.params.iter().enumerate() {
        if i < args.len() {
            ctx.values.insert(*v, args[i]);
        }
    }

    let mut instr_values: Vec<(BlockId, usize, Value)> = Vec::new();
    let mut vi = 0;
    for b in &func.blocks {
        for ii in 0..b.instrs.len() {
            if vi < func.value_types.len() {
                let (v, _) = func.value_types[vi];
                instr_values.push((b.id, ii, v));
                vi += 1;
            }
        }
    }

    let mut current_block = func.entry;
    loop {
        let block = func
            .blocks
            .iter()
            .find(|b| b.id == current_block)
            .ok_or_else(|| format!("block not found: {:?}", current_block))?;

        for (ii, (instr, _)) in block.instrs.iter().enumerate() {
            let result_val = instr_values.iter()
                .find(|(bid, iidx, _)| *bid == block.id && *iidx == ii)
                .map(|(_, _, v)| *v);
            eval_instr(ctx, instr, result_val, module, func)?;
        }

        match &block.terminator {
            Some(term) => match term {
                Terminator::Br(target) => current_block = *target,
                Terminator::CondBr(cond, t, f) => {
                    let cv = *ctx.values.get(cond).unwrap_or(&0);
                    current_block = if cv != 0 { *t } else { *f };
                }
                Terminator::Switch(val, default, cases) => {
                    let v = *ctx.values.get(val).unwrap_or(&0);
                    current_block = cases
                        .iter()
                        .find(|(key, _)| *key == v)
                        .map(|(_, b)| *b)
                        .unwrap_or(*default);
                }
                Terminator::Ret(val) => return Ok(val.map(|v| ctx.values[&v])),
                Terminator::Unreachable => return Err("reached unreachable code".to_string()),
            },
            None => return Err("block has no terminator".to_string()),
        }
    }
}

fn eval_instr(
    ctx: &mut EvalContext,
    instr: &Instr,
    result_val: Option<Value>,
    module: &IRModule,
    _func: &IRFunction,
) -> Result<(), String> {
    let res = match instr {
        Instr::ConstInt(n) => *n,
        Instr::ConstFloat(f) => f64::to_bits(*f) as i64,
        Instr::ConstBool(b) => if *b { 1 } else { 0 },
        Instr::ConstNull => 0,
        Instr::ConstString(s) => ctx.strings.iter().position(|x| x == s).unwrap_or(0) as i64,
        Instr::BinOp(op, l, r) => {
            let lv = *ctx.values.get(l).unwrap_or(&0);
            let rv = *ctx.values.get(r).unwrap_or(&0);
            use crate::ast::BinOp;
            match op {
                BinOp::Add => lv.wrapping_add(rv),
                BinOp::Sub => lv.wrapping_sub(rv),
                BinOp::Mul => lv.wrapping_mul(rv),
                BinOp::Div => { if rv == 0 { return Err("division by zero".to_string()); } lv.wrapping_div(rv) }
                BinOp::Rem => { if rv == 0 { return Err("division by zero".to_string()); } lv.wrapping_rem(rv) }
                BinOp::Eq => (lv == rv) as i64,
                BinOp::Ne => (lv != rv) as i64,
                BinOp::Lt => (lv < rv) as i64,
                BinOp::Gt => (lv > rv) as i64,
                BinOp::Le => (lv <= rv) as i64,
                BinOp::Ge => (lv >= rv) as i64,
                BinOp::And => if lv != 0 && rv != 0 { 1 } else { 0 },
                BinOp::Or => if lv != 0 || rv != 0 { 1 } else { 0 },
            }
        }
        Instr::UnOp(op, operand) => {
            let v = *ctx.values.get(operand).unwrap_or(&0);
            use crate::ast::UnOp;
            match op { UnOp::Neg => v.wrapping_neg(), UnOp::Not => (v == 0) as i64 }
        }
        Instr::Cmp(op, l, r) => {
            let lv = *ctx.values.get(l).unwrap_or(&0);
            let rv = *ctx.values.get(r).unwrap_or(&0);
            use crate::ast::BinOp;
            match op {
                BinOp::Eq => (lv == rv) as i64, BinOp::Ne => (lv != rv) as i64,
                BinOp::Lt => (lv < rv) as i64, BinOp::Gt => (lv > rv) as i64,
                BinOp::Le => (lv <= rv) as i64, BinOp::Ge => (lv >= rv) as i64,
                _ => 0,
            }
        }
        Instr::Alloca(_ty) => ctx.alloc(),
        Instr::Load(ptr) => { let addr = *ctx.values.get(ptr).unwrap_or(&0); *ctx.heap.get(&addr).unwrap_or(&0) }
        Instr::Store(ptr, val) => { let addr = *ctx.values.get(ptr).unwrap_or(&0); let v = *ctx.values.get(val).unwrap_or(&0); ctx.heap.insert(addr, v); 0 }
        Instr::ExtractValue { aggregate, index } => { let base = *ctx.values.get(aggregate).unwrap_or(&0); *ctx.heap.get(&(base + *index as i64)).unwrap_or(&0) }
        Instr::InsertValue { aggregate, element, index } => { let base = *ctx.values.get(aggregate).unwrap_or(&0); let v = *ctx.values.get(element).unwrap_or(&0); ctx.heap.insert(base + *index as i64, v); 0 }
        Instr::Call { args, .. } => { let _ = args; 0 }
        Instr::UserCall { name, args, .. } => {
            let a: Vec<i64> = args.iter().map(|v| *ctx.values.get(v).unwrap_or(&0)).collect();
            let func = module.functions.iter().find(|f| f.name == name.as_str());
            match func {
                Some(f) => match exec_func(ctx, f, &a, module)? {
                    Some(v) => v,
                    None => 0,
                },
                None => 0,
            }
        }
        Instr::IntrinsicCall { intrinsic, args, .. } => {
            let a: Vec<i64> = args.iter().map(|v| *ctx.values.get(v).unwrap_or(&0)).collect();
            handle_runtime_call(ctx, intrinsic_to_name(intrinsic), &a, module)?
        }
        Instr::ExternCall { name, args, .. } => {
            let a: Vec<i64> = args.iter().map(|v| *ctx.values.get(v).unwrap_or(&0)).collect();
            handle_runtime_call(ctx, name, &a, module)?
        }
        Instr::Phi(_) => 0,
        _ => 0,
    };

    if let Some(v) = result_val {
        ctx.values.insert(v, res);
    }
    Ok(())
}

fn intrinsic_to_name(intrinsic: &Intrinsic) -> &str {
    match intrinsic {
        Intrinsic::AiCall => "aial_rt_ai_call",
        Intrinsic::AiCallMany => "aial_rt_ai_call_many",
        Intrinsic::AiCallRace => "aial_rt_ai_call_race",
        Intrinsic::ContextNew => "aial_rt_ctx_new",
        Intrinsic::ContextCurrent => "aial_rt_ctx_current",
        Intrinsic::ContextBudget => "aial_rt_ctx_budget",
        Intrinsic::ExtractAiText => "aial_rt_extract_ai_text",
        Intrinsic::ExtractAiVariant => "aial_rt_extract_ai_variant",
        Intrinsic::ExtractAiUsage => "aial_rt_extract_ai_usage",
        Intrinsic::ExtractAiReasoning => "aial_rt_extract_ai_reasoning",
        Intrinsic::ToolDispatch => "aial_rt_tool_dispatch",
        Intrinsic::CapCheck => "aial_rt_cap_check",
        Intrinsic::ActorSpawn => "aial_rt_actor_spawn",
        Intrinsic::ActorSpawnHandler => "aial_rt_actor_spawn_handler",
        Intrinsic::ActorSend => "aial_rt_actor_send",
        Intrinsic::ActorReceive => "aial_rt_actor_receive",
        Intrinsic::ActorTryReceive => "aial_rt_actor_try_receive",
        Intrinsic::ActorRecvTimeout => "aial_rt_actor_recv_timeout",
        Intrinsic::ActorError => "aial_rt_actor_error",
        Intrinsic::Println => "aial_rt_println",
        Intrinsic::PrivacySensitive => "aial_rt_privacy_sensitive",
        Intrinsic::ContextForget => "aial_rt_ctx_forget",
        Intrinsic::ContextAddMessage => "aial_rt_ctx_add_message",
        Intrinsic::ContextReflect => "aial_rt_ctx_reflect",
        Intrinsic::StrLen => "aial_rt_strlen",
        Intrinsic::StrConcat => "aial_rt_strcat",
        Intrinsic::StrSlice => "aial_rt_strslice",
        Intrinsic::StrChr => "aial_rt_strchr",
        Intrinsic::StrEq => "aial_rt_str_eq",
        Intrinsic::StartsWith => "aial_rt_starts_with",
        Intrinsic::FileRead => "aial_rt_file_read",
        Intrinsic::FileWrite => "aial_rt_file_write",
        Intrinsic::FileAppend => "aial_rt_file_append",
        Intrinsic::FilePatch => "aial_rt_file_patch",
        Intrinsic::EnumCreate => "aial_rt_enum_create",
        Intrinsic::HttpGet => "aial_rt_http_get",
        Intrinsic::HttpStatus => "aial_rt_http_status",
        Intrinsic::HttpText => "aial_rt_http_text",
        Intrinsic::JsonParse => "aial_rt_json_parse",
        Intrinsic::JsonGet => "aial_rt_json_get",
        Intrinsic::JsonGetOr => "aial_rt_json_get_or",
        Intrinsic::JsonType => "aial_rt_json_type",
        Intrinsic::JsonToString => "aial_rt_json_stringify",
        Intrinsic::JsonValueToString => "aial_rt_json_value_to_string",
        Intrinsic::JsonToInt => "aial_rt_json_to_int",
        Intrinsic::JsonToFloat => "aial_rt_json_to_float",
        Intrinsic::JsonArrayLen => "aial_rt_json_array_len",
        Intrinsic::JsonArrayGet => "aial_rt_json_array_get",
        Intrinsic::HttpPost => "aial_rt_http_post",
        Intrinsic::HttpPostJson => "aial_rt_http_post_json",
        Intrinsic::HttpHeaderMap => "aial_rt_http_header_map",
        Intrinsic::HttpHeaderSet => "aial_rt_http_header_set",
        Intrinsic::HttpStart => "aial_rt_http_start",
        Intrinsic::HttpListen => "aial_rt_http_listen",
        Intrinsic::HttpRespond => "aial_rt_http_respond",
        Intrinsic::HttpRequestBody => "aial_rt_http_body",
        Intrinsic::HttpMethod => "aial_rt_http_method",
        Intrinsic::HttpPath => "aial_rt_http_path",
        Intrinsic::HttpQuery => "aial_rt_http_query",
        Intrinsic::HttpHeader => "aial_rt_http_header",
        Intrinsic::HttpUrl => "aial_rt_http_url",
        Intrinsic::HttpStatusText => "aial_rt_http_status_text",
        Intrinsic::HttpOk => "aial_rt_http_ok",
        Intrinsic::HttpJson => "aial_rt_http_json",
        Intrinsic::HttpHtml => "aial_rt_http_html",
        Intrinsic::HttpServe => "aial_rt_http_serve",
        Intrinsic::HtmlEscape => "aial_rt_html_escape",
        Intrinsic::AiStreamStart => "aial_rt_ai_stream_start",
        Intrinsic::AiStreamRead => "aial_rt_ai_stream_read",
        Intrinsic::AiCallRaw => "aial_rt_ai_call_raw",
        Intrinsic::IoReadln => "aial_rt_io_readln",
        Intrinsic::IoReadlnTimeout => "aial_rt_io_readln_timeout",
        Intrinsic::IoReadkey => "aial_rt_io_readkey",
        Intrinsic::IoReadkeyTimeout => "aial_rt_io_readkey_timeout",
        Intrinsic::IoRawMode => "aial_rt_io_raw_mode",
        Intrinsic::Print => "aial_rt_print",
        Intrinsic::CtxOpenMemory => "aial_rt_ctx_open_memory",
        Intrinsic::CtxSaveMessage => "aial_rt_ctx_save_message",
        Intrinsic::CtxLoadMessages => "aial_rt_ctx_load_messages",
        Intrinsic::CtxLoadMessagesSince => "aial_rt_ctx_load_messages_since",
        Intrinsic::CtxCloseMemory => "aial_rt_ctx_close_memory",
        Intrinsic::CtxLastError => "aial_rt_ctx_last_error",
        Intrinsic::TimeNow => "aial_rt_time_now",
        Intrinsic::TimeSleep => "aial_rt_time_sleep",
        Intrinsic::FfiLoad => "aial_rt_ffi_load",
        Intrinsic::FfiCall => "aial_rt_ffi_call",
        Intrinsic::FfiClose => "aial_rt_ffi_close",
        Intrinsic::MapNew => "aial_rt_map_new",
        Intrinsic::MapSet => "aial_rt_map_set",
        Intrinsic::MapGet => "aial_rt_map_get",
        Intrinsic::MapHas => "aial_rt_map_has",
        Intrinsic::KeySet => "aial_rt_key_set",
        Intrinsic::KeyExists => "aial_rt_key_exists",
        Intrinsic::KeyDelete => "aial_rt_key_delete",
        Intrinsic::MapRemove => "aial_rt_map_remove",
        Intrinsic::TokenEstimate => "aial_rt_token_estimate",
        Intrinsic::HeapNew => "aial_rt_heap_new",
        Intrinsic::HeapPush => "aial_rt_heap_push",
        Intrinsic::HeapPop => "aial_rt_heap_pop",
        Intrinsic::HeapPeek => "aial_rt_heap_peek",
        Intrinsic::HeapLen => "aial_rt_heap_len",
        Intrinsic::ArrayNew => "aial_rt_array_new",
        Intrinsic::ArrayPush => "aial_rt_array_push",
        Intrinsic::ArraySort => "aial_rt_array_sort",
        Intrinsic::ArrayGet => "aial_rt_array_get",
        Intrinsic::ArrayLen => "aial_rt_array_len",
    }
}

/// Look up a string by ID — tries string_store first (runtime), then strings table (compile-time)
fn lookup_string(ctx: &EvalContext, id: usize) -> String {
    ctx.string_store.get(&(id as i64)).cloned()
        .or_else(|| ctx.strings.get(id).cloned())
        .unwrap_or_default()
}

fn handle_runtime_call(
    ctx: &mut EvalContext,
    name: &str,
    args: &[i64],
    _module: &IRModule,
) -> Result<i64, String> {
    match name {
        "aial_rt_ai_call" => {
            let model_code = args.first().copied().unwrap_or(0);
            let ctx_id = args.get(1).copied().unwrap_or(0);
            let prompt_idx = args.get(2).copied().unwrap_or(0) as usize;
            let prompt = ctx.strings.get(prompt_idx).map(|s| s.as_str()).unwrap_or("");
            let temp_bits = args.get(3).copied().unwrap_or(0);
            let temperature = f64::from_bits(temp_bits as u64);
            let max_tokens = args.get(4).copied().unwrap_or(1024);
            let format_code = args.get(5).copied().unwrap_or(0);
            let (provider, model_name) = crate::capability::resolve_model(model_code);

            let budget_ok = ctx.contexts.get(&ctx_id).map_or(true, |s| {
                if s.hard_cap && s.tokens_used >= s.token_budget { false } else { true }
            });
            if !budget_ok {
                return Err(format!(
                    "token budget exhausted: {} used, {} budget",
                    ctx.contexts.get(&ctx_id).map(|s| s.tokens_used).unwrap_or(0),
                    ctx.contexts.get(&ctx_id).map(|s| s.token_budget).unwrap_or(0),
                ));
            }

            // #5: compile-time token estimation (heuristic: chars/3 ~= tokens)
            let est_tokens = (prompt.len() as f64 / 3.0) as i64 + max_tokens;
            let budget_str = ctx.contexts.get(&ctx_id).map(|s| format!("{}/{}", s.tokens_used, s.token_budget)).unwrap_or("?".to_string());
            eprintln!("[AI Call] provider={}, model={}, prompt=\"{}\" (est {} tokens, budget {})", provider, model_name, prompt, est_tokens, budget_str);
            // #7: cycle detection — warn if tokens_used suggests rapid consecutive calls
            if let Some(state) = ctx.contexts.get(&ctx_id) {
                if state.tokens_used > 0 && state.strategy.contains("sliding_window") {
                    eprintln!("[cycle] potential feedback loop: consecutive ask calls on same context");
                }
            }

            let mock_mode = std::env::var("AIAL_MOCK").is_ok();
            let text = if mock_mode {
                format!("[{} mock] response from model {}", provider, model_name)
            } else {
                match crate::key_manager::get_key(&provider) {
                    Ok(api_key) => match call_ai_api(&provider, &model_name, &api_key, prompt, temperature, max_tokens, format_code) {
                        Ok(r) => r,
                        Err(e) => format!("(API call failed: {})", e),
                    },
                    Err(e) => format!("(no key configured: {} — set AIAL_MOCK=1 for mock mode)", e),
                }
            };

            let usage_tokens = if mock_mode { max_tokens / 2 } else { max_tokens };
            if let Some(state) = ctx.contexts.get_mut(&ctx_id) {
                state.tokens_used += usage_tokens;
                // #9: record in causal DAG
                let msg_id = state.message_counter;
                state.message_counter += 1;
                state.cause_chain.push((msg_id, format!("ask: {}", &prompt[..prompt.len().min(60)])));
            }

            let resp_addr = ctx.alloc_block(4);
            let text_addr = ctx.alloc();
            ctx.string_store.insert(text_addr, text);
            ctx.heap.insert(resp_addr, 0);
            ctx.heap.insert(resp_addr + 1, text_addr);
            ctx.heap.insert(resp_addr + 2, 0);
            ctx.heap.insert(resp_addr + 3, usage_tokens);
            Ok(resp_addr)
        }
        "aial_rt_ctx_new" => {
            let prompt_idx = args.first().copied().unwrap_or(0) as usize;
            let system_prompt = ctx.strings.get(prompt_idx).map(|s| s.clone()).unwrap_or_default();
            let token_budget = args.get(1).copied().unwrap_or(4096);
            let strat_idx = args.get(2).copied().unwrap_or(0) as usize;
            let strategy = if strat_idx == 0 { String::new() } else { ctx.strings.get(strat_idx).cloned().unwrap_or_default() };
            let window_size = args.get(3).copied().unwrap_or(0);
            let id = ctx.next_ctx_id;
            ctx.next_ctx_id += 1;
            ctx.contexts.insert(id, ContextState {
                id, system_prompt, token_budget, tokens_used: 0, hard_cap: true,
                strategy, window_size,
                cause_chain: vec![(-1, "context_created".to_string())],
                message_counter: 0,
                messages: Vec::new(),
            });
            Ok(id)
        }
        "aial_rt_ctx_current" => Ok(1),
        "aial_rt_ctx_budget" => {
            let ctx_id = args.first().copied().unwrap_or(0);
            match ctx.contexts.get(&ctx_id) {
                Some(s) => Ok(s.token_budget - s.tokens_used),
                None => Ok(0),
            }
        }
        "aial_rt_extract_ai_text" => {
            let text_ptr = *ctx.heap.get(&(args.first().copied().unwrap_or(0) + 1)).unwrap_or(&0);
            Ok(text_ptr)
        }
        "aial_rt_extract_ai_variant" => {
            let resp_addr = args.first().copied().unwrap_or(0);
            Ok(*ctx.heap.get(&resp_addr).unwrap_or(&-1))
        }
        "aial_rt_extract_ai_usage" => {
            let resp_addr = args.first().copied().unwrap_or(0);
            Ok(*ctx.heap.get(&(resp_addr + 3)).unwrap_or(&0))
        }
        "aial_rt_extract_ai_reasoning" => Ok(0),
        "aial_rt_println" => {
            let text_addr = args.first().copied().unwrap_or(0);
            let text = lookup_string(ctx, text_addr as usize);
            // Taint check: if text contains sensitive data, mask or warn
            let is_tainted = ctx.tainted.contains(&text_addr) || ctx.tainted.iter().any(|&t| {
                ctx.string_store.get(&t).map_or(false, |s| text.contains(s))
            });
            if is_tainted {
                eprintln!("[privacy] WARNING: printing tainted/sensitive data");
            }
            println!("{}", text);
            Ok(0)
        }
        "aial_rt_privacy_sensitive" => {
            let val = args.first().copied().unwrap_or(0);
            ctx.tainted.insert(val);
            eprintln!("[privacy] value marked as sensitive (tainted set: {} items)", ctx.tainted.len());
            Ok(val)
        }
        "aial_rt_ctx_add_message" => {
            let ctx_id = args.first().copied().unwrap_or(0);
            let role_idx = args.get(1).copied().unwrap_or(0);
            let content_idx = args.get(2).copied().unwrap_or(0);
            let role = lookup_string(&ctx, role_idx as usize);
            let content = lookup_string(&ctx, content_idx as usize);
            if let Some(state) = ctx.contexts.get_mut(&ctx_id) {
                let msg = format!("[{}] {}", role, content);
                state.messages.push(msg);
            }
            Ok(ctx_id)
        }
        "aial_rt_ctx_forget" => {
            let ctx_id = args.first().copied().unwrap_or(0);
            let cause_id = args.get(1).copied().unwrap_or(0);
            if let Some(state) = ctx.contexts.get_mut(&ctx_id) {
                let old_len = state.cause_chain.len();
                // Remove the cause and all entries derived from it (higher IDs)
                state.cause_chain.retain(|(id, _)| *id < cause_id || *id > cause_id + 10);
                eprintln!("[forget] causal pruning: context {}, removed {} entries (cause_id={})",
                    ctx_id, old_len - state.cause_chain.len(), cause_id);
            }
            Ok(0)
        }
        "aial_rt_ctx_reflect" => {
            let ctx_id = args.first().copied().unwrap_or(0);
            let entries: Vec<String> = ctx.contexts.get(&ctx_id).map(|s| {
                s.cause_chain.iter().map(|(id, desc)| format!("  [{id}] {desc}")).collect()
            }).unwrap_or_default();
            let prompt = format!(
                "[Self-Correction]\n\
                 Recent interaction log:\n{}\n\
                 Review the above for: 1) factual errors, 2) logical contradictions,\n\
                 3) missed edge cases, 4) consistency with prior responses.",
                entries.join("\n")
            );
            eprintln!("[reflect] generated reflection prompt ({} entries)", entries.len());
            let text_addr = ctx.alloc();
            ctx.string_store.insert(text_addr, prompt);
            Ok(text_addr)
        }
        "aial_rt_tool_dispatch" => {
            let tool_name_idx = args.first().copied().unwrap_or(0) as usize;
            let tool_name = ctx.strings.get(tool_name_idx).map(|s| s.as_str()).unwrap_or("?");
            // Look up registered tool and return (simulated) result
            let found = ctx.tools.iter().find(|t| t.name == tool_name);
            match found {
                Some(t) => {
                    eprintln!("[tool dispatch] calling `{}`: {}", t.name, t.description);
                    // Return a mock result
                    let result = format!("{{}}\"result\": \"simulated_{}_output\"", t.name);
                    let text_addr = ctx.alloc();
                    ctx.string_store.insert(text_addr, result);
                    Ok(text_addr)
                }
                None => {
                    eprintln!("[tool dispatch] tool `{}` not found in {} registered tools", tool_name, ctx.tools.len());
                    Ok(0)
                }
            }
        }
        "aial_rt_strlen" => {
            let idx = args.first().copied().unwrap_or(0) as usize;
            let s = lookup_string(ctx, idx);
            Ok(s.len() as i64)
        }
        "aial_rt_strcat" => {
            let a = lookup_string(ctx, args.first().copied().unwrap_or(0) as usize);
            let b = lookup_string(ctx, args.get(1).copied().unwrap_or(0) as usize);
            let result = a + &b;
            let addr = ctx.alloc();
            ctx.string_store.insert(addr, result);
            Ok(addr)
        }
        "aial_rt_str_eq" => {
            let a = lookup_string(ctx, args.first().copied().unwrap_or(0) as usize);
            let b = lookup_string(ctx, args.get(1).copied().unwrap_or(0) as usize);
            Ok(if a == b { 1 } else { 0 })
        }
        "aial_rt_starts_with" => {
            let s = lookup_string(ctx, args.first().copied().unwrap_or(0) as usize);
            let pre = lookup_string(ctx, args.get(1).copied().unwrap_or(0) as usize);
            Ok(if s.starts_with(&pre) { 1 } else { 0 })
        }
        "aial_rt_strchr" => {
            let s = lookup_string(ctx, args.first().copied().unwrap_or(0) as usize);
            let idx = args.get(1).copied().unwrap_or(0) as usize;
            Ok(s.chars().nth(idx).map(|c| c as i64).unwrap_or(-1))
        }
        "aial_rt_strslice" => {
            let s = lookup_string(ctx, args.first().copied().unwrap_or(0) as usize);
            let start = args.get(1).copied().unwrap_or(0) as usize;
            let len = args.get(2).copied().unwrap_or(0) as usize;
            let slice: String = s.chars().skip(start).take(len).collect();
            let addr = ctx.alloc();
            ctx.string_store.insert(addr, slice);
            Ok(addr)
        }
        "aial_rt_file_read" => {
            let idx = args.first().copied().unwrap_or(0) as usize;
            let path = ctx.strings.get(idx).map(|s| s.as_str()).unwrap_or("");
            let content = std::fs::read_to_string(path).unwrap_or_else(|e| format!("[read error: {}]", e));
            let addr = ctx.alloc();
            ctx.string_store.insert(addr, content);
            Ok(addr)
        }
        "aial_rt_file_write" => {
            let pi = args.first().copied().unwrap_or(0) as usize;
            let ci = args.get(1).copied().unwrap_or(0) as usize;
            let path = ctx.strings.get(pi).map(|s| s.as_str()).unwrap_or("");
            let content = ctx.strings.get(ci).cloned().unwrap_or_default();
            eprintln!("[file::write] {} ({} bytes)", path, content.len());
            if let Some(parent) = Path::new(path).parent() { let _ = std::fs::create_dir_all(parent); }
            let _ = std::fs::write(path, &content);
            Ok(0)
        }
        "aial_rt_file_append" => {
            let pi = args.first().copied().unwrap_or(0) as usize;
            let ci = args.get(1).copied().unwrap_or(0) as usize;
            let path = ctx.strings.get(pi).map(|s| s.as_str()).unwrap_or("");
            let content = ctx.strings.get(ci).cloned().unwrap_or_default();
            eprintln!("[file::append] {} ({} bytes)", path, content.len());
            use std::io::Write;
            let _ = std::fs::OpenOptions::new().create(true).append(true).open(path)
                .map(|mut f| { let _ = f.write_all(content.as_bytes()); });
            Ok(0)
        }
        "aial_rt_file_patch" => {
            let pi = args.first().copied().unwrap_or(0) as usize;
            let oi = args.get(1).copied().unwrap_or(0) as usize;
            let ni = args.get(2).copied().unwrap_or(0) as usize;
            let path = ctx.strings.get(pi).map(|s| s.as_str()).unwrap_or("");
            let old = ctx.strings.get(oi).cloned().unwrap_or_default();
            let new = ctx.strings.get(ni).cloned().unwrap_or_default();
            // Atomic: read → replace → write .tmp → rename
            let content = std::fs::read_to_string(path).unwrap_or_default();
            let patched = content.replace(&old, &new);
            let tmp = format!("{}.aial_tmp", path);
            let _ = std::fs::write(&tmp, &patched);
            let _ = std::fs::rename(&tmp, path);
            eprintln!("[file::patch] {} replaced {} occurrences", path, content.matches(&old).count());
            Ok(0)
        }
        "aial_rt_enum_create" => {
            // Alloc space for variant: [fields...] at offsets 0, 1, 2...
            let n = args.len().saturating_sub(2); // args after type_name + variant_name
            let ptr = ctx.alloc_block(n.max(1));
            for (i, val) in args.iter().skip(2).enumerate() {
                ctx.heap.insert(ptr + i as i64, *val);
            }
            Ok(ptr)
        }
        "aial_rt_http_get" => {
            let url = lookup_string(ctx, args.first().copied().unwrap_or(0) as usize);
            // HTTP response: [status, body_ptr, headers_ptr] at offsets 0,1,2
            let resp_ptr = ctx.alloc_block(3);
            match reqwest::blocking::get(&url) {
                Ok(resp) => {
                    let status = resp.status().as_u16() as i64;
                    let body = resp.text().unwrap_or_default();
                    let body_ptr = ctx.alloc();
                    ctx.string_store.insert(body_ptr, body);
                    ctx.heap.insert(resp_ptr, status);
                    ctx.heap.insert(resp_ptr + 1, body_ptr);
                    ctx.heap.insert(resp_ptr + 2, 0); // headers placeholder
                    Ok(resp_ptr)
                }
                Err(e) => {
                    let err_msg = format!("[http error: {}]", e);
                    let body_ptr = ctx.alloc();
                    ctx.string_store.insert(body_ptr, err_msg);
                    ctx.heap.insert(resp_ptr, 0); // status 0 = error
                    ctx.heap.insert(resp_ptr + 1, body_ptr);
                    ctx.heap.insert(resp_ptr + 2, 0);
                    Ok(resp_ptr)
                }
            }
        }
        "aial_rt_http_status" => {
            let resp = args.first().copied().unwrap_or(0);
            Ok(ctx.heap.get(&resp).copied().unwrap_or(0))
        }
        "aial_rt_http_text" => {
            let resp = args.first().copied().unwrap_or(0);
            Ok(ctx.heap.get(&(resp + 1)).copied().unwrap_or(0))
        }
        "aial_rt_json_parse" => {
            let text = lookup_string(ctx, args.first().copied().unwrap_or(0) as usize);
            let val_ptr = ctx.alloc_block(5); // [type, value/bool_val/f64_bits, size, ptr, flags]
            match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(v) => { write_json_to_heap(ctx, val_ptr, &v); Ok(val_ptr) }
                Err(e) => {
                    // JsonError: type=(-1), value=error_msg_ptr
                    ctx.heap.insert(val_ptr, -1);
                    let err_ptr = ctx.alloc();
                    ctx.string_store.insert(err_ptr, e.to_string());
                    ctx.heap.insert(val_ptr + 1, err_ptr);
                    Ok(val_ptr)
                }
            }
        }
        "aial_rt_json_get" => {
            let val_ptr = args.first().copied().unwrap_or(0);
            let key = lookup_string(ctx, args.get(1).copied().unwrap_or(0) as usize);
            let result = json_lookup(ctx, val_ptr, &key);
            match result {
                Some(r) => Ok(r),
                None => {
                    let null_ptr = ctx.alloc_block(5);
                    ctx.heap.insert(null_ptr, 0); // type 0 = null
                    Ok(null_ptr)
                }
            }
        }
        "aial_rt_json_get_or" => {
            let val_ptr = args.first().copied().unwrap_or(0);
            let key = lookup_string(ctx, args.get(1).copied().unwrap_or(0) as usize);
            let default = args.get(2).copied().unwrap_or(0);
            let result = json_lookup(ctx, val_ptr, &key);
            match result {
                Some(r) => Ok(r),
                None => Ok(default),
            }
        }
        "aial_rt_json_type" => {
            let val_ptr = args.first().copied().unwrap_or(0);
            Ok(ctx.heap.get(&val_ptr).copied().unwrap_or(0))
        }
        "aial_rt_http_post" => {
            let url = lookup_string(ctx, args.first().copied().unwrap_or(0) as usize);
            let body = lookup_string(ctx, args.get(1).copied().unwrap_or(0) as usize);
            let resp_ptr = ctx.alloc_block(3);
            let client = reqwest::blocking::Client::new();
            match client.post(&url).body(body).send() {
                Ok(resp) => {
                    let status = resp.status().as_u16() as i64;
                    let text = resp.text().unwrap_or_default();
                    let body_ptr = ctx.alloc(); ctx.string_store.insert(body_ptr, text);
                    ctx.heap.insert(resp_ptr, status); ctx.heap.insert(resp_ptr + 1, body_ptr); ctx.heap.insert(resp_ptr + 2, 0);
                }
                Err(e) => {
                    let err = format!("[http error: {}]", e);
                    let body_ptr = ctx.alloc(); ctx.string_store.insert(body_ptr, err);
                    ctx.heap.insert(resp_ptr, 0); ctx.heap.insert(resp_ptr + 1, body_ptr);
                }
            }
            Ok(resp_ptr)
        }
        "aial_rt_http_post_json" => {
            let url = lookup_string(ctx, args.first().copied().unwrap_or(0) as usize);
            let val_ptr = args.get(1).copied().unwrap_or(0);
            let json_str = json_value_to_string(ctx, val_ptr);
            let resp_ptr = ctx.alloc_block(3);
            let client = reqwest::blocking::Client::new();
            match client.post(&url).header("Content-Type", "application/json").body(json_str).send() {
                Ok(resp) => {
                    let status = resp.status().as_u16() as i64;
                    let text = resp.text().unwrap_or_default();
                    let body_ptr = ctx.alloc(); ctx.string_store.insert(body_ptr, text);
                    ctx.heap.insert(resp_ptr, status); ctx.heap.insert(resp_ptr + 1, body_ptr);
                }
                Err(e) => {
                    let err = format!("[http error: {}]", e);
                    let body_ptr = ctx.alloc(); ctx.string_store.insert(body_ptr, err);
                    ctx.heap.insert(resp_ptr, 0); ctx.heap.insert(resp_ptr + 1, body_ptr);
                }
            }
            Ok(resp_ptr)
        }
        "aial_rt_http_header_map" => {
            let ptr = ctx.alloc_block(128); // room for key-value pairs
            ctx.heap.insert(ptr, 0); // count
            Ok(ptr)
        }
        "aial_rt_http_header_set" => {
            let map = args.first().copied().unwrap_or(0);
            let key = lookup_string(ctx, args.get(1).copied().unwrap_or(0) as usize);
            let val = lookup_string(ctx, args.get(2).copied().unwrap_or(0) as usize);
            let n = ctx.heap.get(&map).copied().unwrap_or(0);
            let idx = n * 2 + 1;
            let kp = ctx.alloc(); ctx.string_store.insert(kp, key);
            let vp = ctx.alloc(); ctx.string_store.insert(vp, val);
            ctx.heap.insert(map + idx, kp);
            ctx.heap.insert(map + idx + 1, vp);
            ctx.heap.insert(map, n + 1);
            Ok(map)
        }
        "aial_rt_json_stringify" => {
            let val_ptr = args.first().copied().unwrap_or(0);
            let s = json_value_to_string(ctx, val_ptr);
            let ptr = ctx.alloc(); ctx.string_store.insert(ptr, s);
            Ok(ptr)
        }
        "aial_rt_json_value_to_string" => {
            let val_ptr = args.first().copied().unwrap_or(0);
            let tag = ctx.heap.get(&val_ptr).copied().unwrap_or(0);
            let s = match tag {
                3 => lookup_string(ctx, ctx.heap.get(&(val_ptr + 1)).copied().unwrap_or(0) as usize),
                2 => format!("{}", f64::from_bits(ctx.heap.get(&(val_ptr + 2)).copied().unwrap_or(0) as u64)),
                1 => (ctx.heap.get(&(val_ptr + 1)).copied().unwrap_or(0) != 0).to_string(),
                0 => "null".to_string(),
                _ => json_value_to_string(ctx, val_ptr),
            };
            let ptr = ctx.alloc(); ctx.string_store.insert(ptr, s);
            Ok(ptr)
        }
        "aial_rt_json_to_int" => {
            let val_ptr = args.first().copied().unwrap_or(0);
            let tag = ctx.heap.get(&val_ptr).copied().unwrap_or(0);
            let v = match tag {
                2 => f64::from_bits(ctx.heap.get(&(val_ptr + 2)).copied().unwrap_or(0) as u64) as i64,
                1 => ctx.heap.get(&(val_ptr + 1)).copied().unwrap_or(0),
                _ => 0,
            };
            Ok(v)
        }
        "aial_rt_json_to_float" => {
            let val_ptr = args.first().copied().unwrap_or(0);
            let tag = ctx.heap.get(&val_ptr).copied().unwrap_or(0);
            let f = match tag {
                2 => f64::from_bits(ctx.heap.get(&(val_ptr + 2)).copied().unwrap_or(0) as u64),
                1 => ctx.heap.get(&(val_ptr + 1)).copied().unwrap_or(0) as f64,
                _ => 0.0,
            };
            Ok(f.to_bits() as i64)
        }
        "aial_rt_json_array_len" => {
            let val_ptr = args.first().copied().unwrap_or(0);
            let tag = ctx.heap.get(&val_ptr).copied().unwrap_or(0);
            Ok(if tag == 4 { ctx.heap.get(&(val_ptr + 2)).copied().unwrap_or(0) } else { 0 })
        }
        "aial_rt_json_array_get" => {
            let val_ptr = args.first().copied().unwrap_or(0);
            let idx = args.get(1).copied().unwrap_or(0);
            let arr_ptr = ctx.heap.get(&(val_ptr + 3)).copied().unwrap_or(0);
            match ctx.heap.get(&(arr_ptr + idx)).copied() {
                Some(v) => Ok(v),
                None => {
                    let null_ptr = ctx.alloc_block(5); ctx.heap.insert(null_ptr, 0); Ok(null_ptr)
                }
            }
        }
        "aial_rt_html_escape" => {
            let text = lookup_string(ctx, args.first().copied().unwrap_or(0) as usize);
            let escaped = text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;");
            let ptr = ctx.alloc(); ctx.string_store.insert(ptr, escaped); Ok(ptr)
        }
        "aial_rt_ai_stream_start" => {
            let stream_ptr = ctx.alloc_block(4);
            ctx.heap.insert(stream_ptr, 0); // read position = 0
            ctx.heap.insert(stream_ptr + 1, 1); // 1 "token" (the full response text)
            // Store AI call args so read can use them
            let prompt_idx = args.get(2).copied().unwrap_or(0);
            ctx.heap.insert(stream_ptr + 2, prompt_idx); // prompt idx for the call
            ctx.heap.insert(stream_ptr + 3, 0); // "done" flag
            Ok(stream_ptr)
        }
        "aial_rt_ai_call_raw" => {
            // Bare API call — no capability check, no context, no budget
            let model = args.first().copied().unwrap_or(0);
            let prompt_idx = args.get(1).copied().unwrap_or(0) as usize;
            let _max_tokens = args.get(2).copied().unwrap_or(256);
            let prompt = lookup_string(ctx, prompt_idx);
            // Use AIAL_MOCK if available, else attempt real API call
            let text = if std::env::var("AIAL_MOCK").is_ok() {
                format!("[mock] {}", prompt)
            } else {
                format!("[ai_call_raw] model={} prompt={}", model, prompt)
            };
            let ptr = ctx.alloc(); ctx.string_store.insert(ptr, text); Ok(ptr)
        }
        "aial_rt_ai_stream_read" => {
            let stream_ptr = args.first().copied().unwrap_or(0);
            let pos = ctx.heap.get(&stream_ptr).copied().unwrap_or(0);
            if pos > 0 { let empty = ctx.alloc(); ctx.string_store.insert(empty, String::new()); return Ok(empty); }
            ctx.heap.insert(stream_ptr, 1); // mark as read
            let prompt_idx = ctx.heap.get(&(stream_ptr + 2)).copied().unwrap_or(0);
            let text = lookup_string(ctx, prompt_idx as usize);
            let ptr = ctx.alloc(); ctx.string_store.insert(ptr, text); Ok(ptr)
        }
        "aial_rt_io_readln" => {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
            let ptr = ctx.alloc();
            ctx.string_store.insert(ptr, input.trim_end().to_string());
            Ok(ptr)
        }
        "aial_rt_io_readln_timeout" => {
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input);
            let ptr = ctx.alloc();
            ctx.string_store.insert(ptr, input.trim_end().to_string());
            Ok(ptr)
        }
        "aial_rt_print" => {
            let idx = args.first().copied().unwrap_or(0) as usize;
            let text = lookup_string(ctx, idx);
            use std::io::Write;
            print!("{}", text);
            std::io::stdout().flush().ok();
            Ok(0)
        }
        "aial_rt_io_readkey" => {
            use std::io::Read;
            let mut buf = [0u8; 1];
            let n = std::io::stdin().read(&mut buf).unwrap_or(0);
            let ptr = ctx.alloc();
            if n == 0 { ctx.string_store.insert(ptr, String::new()); }
            else { ctx.string_store.insert(ptr, (buf[0] as char).to_string()); }
            Ok(ptr)
        }
        "aial_rt_io_readkey_timeout" => {
            use std::os::unix::io::AsRawFd;
            let ms = args.first().copied().unwrap_or(100);
            let fd = std::io::stdin().as_raw_fd();
            let mut fds = [libc::pollfd { fd, events: libc::POLLIN, revents: 0 }];
            let ret = unsafe { libc::poll(fds.as_mut_ptr(), 1, ms as i32) };
            let ptr = ctx.alloc();
            if ret > 0 {
                use std::io::Read;
                let mut buf = [0u8; 1];
                let n = std::io::stdin().read(&mut buf).unwrap_or(0);
                if n > 0 { ctx.string_store.insert(ptr, (buf[0] as char).to_string()); }
                else { ctx.string_store.insert(ptr, String::new()); }
            } else {
                ctx.string_store.insert(ptr, String::new());
            }
            Ok(ptr)
        }
        "aial_rt_io_raw_mode" => {
            // Stub — full termios raw mode requires platform-specific code
            Ok(0)
        }
        "aial_rt_ctx_open_memory" => {
            use std::sync::Mutex;
            let path = lookup_string(ctx, args.first().copied().unwrap_or(0) as usize);
            let conn = rusqlite::Connection::open(&path).map_err(|e| e.to_string())?;
            let db_ptr = ctx.alloc();
            // Create messages table
            conn.execute("CREATE TABLE IF NOT EXISTS messages (id INTEGER PRIMARY KEY AUTOINCREMENT, session TEXT, role TEXT, content TEXT, ts INTEGER DEFAULT (unixepoch()))", []).ok();
            // Store connection as raw pointer
            let boxed = Box::new(Mutex::new(conn));
            ctx.heap.insert(db_ptr, Box::into_raw(boxed) as i64);
            Ok(db_ptr)
        }
        "aial_rt_ctx_save_message" => {
            let db_ptr = args.first().copied().unwrap_or(0);
            let session = lookup_string(ctx, args.get(1).copied().unwrap_or(0) as usize);
            let role = lookup_string(ctx, args.get(2).copied().unwrap_or(0) as usize);
            let content = lookup_string(ctx, args.get(3).copied().unwrap_or(0) as usize);
            let conn_ptr = ctx.heap.get(&db_ptr).copied().unwrap_or(0) as *mut std::sync::Mutex<rusqlite::Connection>;
            if let Some(mutex) = unsafe { conn_ptr.as_ref() } {
                let conn = mutex.lock().unwrap();
                conn.execute("INSERT INTO messages (session, role, content) VALUES (?1, ?2, ?3)", rusqlite::params![session, role, content]).ok();
            }
            Ok(0)
        }
        "aial_rt_ctx_load_messages" => {
            let db_ptr = args.first().copied().unwrap_or(0);
            let session = lookup_string(ctx, args.get(1).copied().unwrap_or(0) as usize);
            let limit = args.get(2).copied().unwrap_or(50);
            let conn_ptr = ctx.heap.get(&db_ptr).copied().unwrap_or(0) as *mut std::sync::Mutex<rusqlite::Connection>;
            let json = if let Some(mutex) = unsafe { conn_ptr.as_ref() } {
                let conn = mutex.lock().unwrap();
                let mut stmt = conn.prepare("SELECT role, content, ts FROM messages WHERE session=?1 ORDER BY id ASC LIMIT ?2").unwrap();
                let rows: Vec<String> = stmt.query_map(rusqlite::params![session, limit], |row| {
                    Ok(format!(r#"{{"role":"{}","content":"{}","ts":{}}}"#, row.get::<_,String>(0)?, row.get::<_,String>(1)?.replace('"', r#"\""#), row.get::<_,i64>(2)?))
                }).unwrap().filter_map(|r| r.ok()).collect();
                format!("[{}]", rows.join(","))
            } else { "[]".to_string() };
            let ptr = ctx.alloc(); ctx.string_store.insert(ptr, json); Ok(ptr)
        }
        "aial_rt_ctx_load_messages_since" => {
            let db_ptr = args.first().copied().unwrap_or(0);
            let session = lookup_string(ctx, args.get(1).copied().unwrap_or(0) as usize);
            let ts = args.get(2).copied().unwrap_or(0);
            let conn_ptr = ctx.heap.get(&db_ptr).copied().unwrap_or(0) as *mut std::sync::Mutex<rusqlite::Connection>;
            let json = if let Some(mutex) = unsafe { conn_ptr.as_ref() } {
                let conn = mutex.lock().unwrap();
                let mut stmt = conn.prepare("SELECT role, content, ts FROM messages WHERE session=?1 AND ts>=?2 ORDER BY id ASC").unwrap();
                let rows: Vec<String> = stmt.query_map(rusqlite::params![session, ts], |row| {
                    Ok(format!(r#"{{"role":"{}","content":"{}","ts":{}}}"#, row.get::<_,String>(0)?, row.get::<_,String>(1)?.replace('"', r#"\""#), row.get::<_,i64>(2)?))
                }).unwrap().filter_map(|r| r.ok()).collect();
                format!("[{}]", rows.join(","))
            } else { "[]".to_string() };
            let ptr = ctx.alloc(); ctx.string_store.insert(ptr, json); Ok(ptr)
        }
        "aial_rt_ctx_close_memory" => {
            let db_ptr = args.first().copied().unwrap_or(0);
            let conn_ptr = ctx.heap.get(&db_ptr).copied().unwrap_or(0) as *mut std::sync::Mutex<rusqlite::Connection>;
            if let Some(mutex) = unsafe { conn_ptr.as_ref() } { let _guard = mutex.lock(); drop(_guard); }
            unsafe { drop(Box::from_raw(conn_ptr)); }
            Ok(0)
        }
        "aial_rt_time_now" => {
            let ptr = ctx.alloc();
            ctx.string_store.insert(ptr, "2026-05-14T00:00:00".to_string());
            Ok(ptr)
        }
        "aial_rt_time_sleep" => {
            let ms = args.first().copied().unwrap_or(0);
            std::thread::sleep(std::time::Duration::from_millis(ms as u64));
            Ok(0)
        }
        "aial_rt_ffi_load" => {
            Err("FFI not available in interpreter — compile with LLVM backend".to_string())
        }
        "aial_rt_ffi_call" => {
            Err("FFI not available in interpreter — compile with LLVM backend".to_string())
        }
        "aial_rt_ffi_close" => {
            Err("FFI not available in interpreter — compile with LLVM backend".to_string())
        }
        "aial_rt_http_start" | "aial_rt_http_listen" | "aial_rt_http_respond" | "aial_rt_http_body" => {
            Err("[http server] not available in interpreter".to_string())
        }
        "aial_rt_ctx_last_error" => {
            let ptr = ctx.alloc();
            ctx.string_store.insert(ptr, String::new());
            Ok(ptr)
        }
        "aial_rt_map_new" => {
            let pid = ctx.alloc();
            ctx.heap.insert(pid, 0); // map id marker
            Ok(pid)
        }
        "aial_rt_map_set" => {
            let base = args[0] as i64 * 10_000;
            let entry_key = base + args[1] as i64;
            ctx.heap.insert(entry_key, args[2] as i64);
            Ok(0)
        }
        "aial_rt_map_get" => {
            let base = args[0] as i64 * 10_000;
            Ok(ctx.heap.get(&(base + args[1] as i64)).copied().unwrap_or(0))
        }
        "aial_rt_map_has" => {
            let base = args[0] as i64 * 10_000;
            Ok(if ctx.heap.contains_key(&(base + args[1] as i64)) { 1 } else { 0 })
        }
        "aial_rt_map_remove" => {
            let base = args[0] as i64 * 10_000;
            ctx.heap.remove(&(base + args[1] as i64));
            Ok(0)
        }
        "aial_rt_token_estimate" => {
            let s = lookup_string(ctx, args[0] as usize);
            // Rough estimate: ASCII ~4 chars/token, CJK ~1.5 chars/token
            let bytes = s.len();
            let cjk = s.chars().filter(|&c| c >= '\u{4E00}' && c <= '\u{9FFF}').count();
            let ascii = bytes - cjk * 3; // approximate CJK as 3 bytes per char
            Ok((ascii as i64 / 4 + cjk as i64 * 2 / 3).max(1))
        }
        "aial_rt_heap_new" => {
            let pid = ctx.alloc();
            let base = pid * 10_000;
            ctx.heap.insert(base, 0); // size
            Ok(pid)
        }
        "aial_rt_heap_push" => {
            let base = args[0] as i64 * 10_000;
            let size = ctx.heap.get(&base).copied().unwrap_or(0);
            let slot_val = base + 1 + size * 2;
            let slot_pri = slot_val + 1;
            ctx.heap.insert(slot_val, args[1] as i64);
            ctx.heap.insert(slot_pri, args[2] as i64);
            ctx.heap.insert(base, size + 1);
            Ok(0)
        }
        "aial_rt_heap_pop" => {
            let base = args[0] as i64 * 10_000;
            let size = ctx.heap.get(&base).copied().unwrap_or(0);
            if size == 0 { return Ok(0); }
            let mut best_idx = 0i64;
            let mut best_pri = i64::MIN;
            for i in 0..size {
                let pri = ctx.heap.get(&(base + 1 + i * 2 + 1)).copied().unwrap_or(i64::MIN);
                if pri > best_pri { best_pri = pri; best_idx = i; }
            }
            let slot = base + 1 + best_idx * 2;
            let val = ctx.heap.get(&slot).copied().unwrap_or(0);
            let last_slot = base + 1 + (size - 1) * 2;
            if best_idx != size - 1 {
                let last_val = ctx.heap.get(&last_slot).copied().unwrap_or(0);
                let last_pri = ctx.heap.get(&(last_slot + 1)).copied().unwrap_or(0);
                ctx.heap.insert(slot, last_val);
                ctx.heap.insert(slot + 1, last_pri);
            }
            ctx.heap.remove(&last_slot);
            ctx.heap.remove(&(last_slot + 1));
            ctx.heap.insert(base, size - 1);
            Ok(val)
        }
        "aial_rt_heap_peek" => {
            let base = args[0] as i64 * 10_000;
            let size = ctx.heap.get(&base).copied().unwrap_or(0);
            if size == 0 { return Ok(0); }
            let mut best_pri = i64::MIN;
            let mut best_val = 0i64;
            for i in 0..size {
                let pri = ctx.heap.get(&(base + 1 + i * 2 + 1)).copied().unwrap_or(i64::MIN);
                if pri > best_pri { best_pri = pri; best_val = ctx.heap.get(&(base + 1 + i * 2)).copied().unwrap_or(0); }
            }
            Ok(best_val)
        }
        "aial_rt_heap_len" => {
            let base = args[0] as i64 * 10_000;
            Ok(ctx.heap.get(&base).copied().unwrap_or(0))
        }
        "aial_rt_array_new" => {
            let pid = ctx.alloc();
            let base = pid * 10_000;
            ctx.heap.insert(base, 0); // size
            Ok(pid)
        }
        "aial_rt_array_push" => {
            let base = args[0] as i64 * 10_000;
            let size = ctx.heap.get(&base).copied().unwrap_or(0);
            ctx.heap.insert(base + 1 + size, args[1] as i64);
            ctx.heap.insert(base, size + 1);
            Ok(0)
        }
        "aial_rt_array_sort" => {
            let base = args[0] as i64 * 10_000;
            let size = ctx.heap.get(&base).copied().unwrap_or(0) as usize;
            if size <= 1 { return Ok(0); }
            let mut items: Vec<(i64, String)> = (0..size).map(|i| {
                let idx = ctx.heap.get(&(base + 1 + i as i64)).copied().unwrap_or(0);
                let s = lookup_string(ctx, idx as usize);
                (idx, s)
            }).collect();
            items.sort_by(|a, b| a.1.cmp(&b.1));
            for (i, (idx, _)) in items.iter().enumerate() {
                ctx.heap.insert(base + 1 + i as i64, *idx);
            }
            Ok(0)
        }
        "aial_rt_array_get" => {
            let base = args[0] as i64 * 10_000;
            Ok(ctx.heap.get(&(base + 1 + args[1] as i64)).copied().unwrap_or(0))
        }
        "aial_rt_array_len" => {
            let base = args[0] as i64 * 10_000;
            Ok(ctx.heap.get(&base).copied().unwrap_or(0))
        }
        "aial_rt_key_set" => Ok(1),
        "aial_rt_key_exists" => Ok(0),
        "aial_rt_key_delete" => Ok(1),
        "aial_rt_cap_check" => Ok(1),
        "aial_rt_actor_spawn" => {
            let pid = ctx.alloc();
            let mbox = ctx.alloc_block(128);
            ctx.heap.insert(mbox, 0);
            ctx.heap.insert(pid, mbox);
            Ok(pid)
        }
        "aial_rt_actor_spawn_handler" => {
            // Interpreter: fallback to mailbox-only (no threads)
            let pid = ctx.alloc();
            let mbox = ctx.alloc_block(128);
            ctx.heap.insert(mbox, 0);
            ctx.heap.insert(pid, mbox);
            let fn_name = lookup_string(ctx, args.first().copied().unwrap_or(0) as usize);
            eprintln!("[actor] spawn_handler({}) -> pid {}", fn_name, pid);
            Ok(pid)
        }
        "aial_rt_actor_send" => {
            let pid = args.first().copied().unwrap_or(0);
            let msg_idx = args.get(1).copied().unwrap_or(0);
            let mbox = ctx.heap.get(&pid).copied().unwrap_or(0);
            if mbox != 0 {
                let count = ctx.heap.get(&mbox).copied().unwrap_or(0);
                ctx.heap.insert(mbox + 1 + count * 2, msg_idx);
                ctx.heap.insert(mbox, count + 1);
            }
            Ok(0)
        }
        "aial_rt_actor_receive" => {
            let pid = args.first().copied().unwrap_or(0);
            let mbox = ctx.heap.get(&pid).copied().unwrap_or(0);
            if mbox == 0 { let ptr = ctx.alloc(); ctx.string_store.insert(ptr, String::new()); return Ok(ptr); }
            // Block until message available (busy-wait for interpreter)
            loop {
                let count = ctx.heap.get(&mbox).copied().unwrap_or(0);
                if count > 0 {
                    let msg = ctx.heap.get(&(mbox + 1)).copied().unwrap_or(0);
                    // Shift remaining messages
                    let mut i = 0;
                    while i < count - 1 {
                        let v = ctx.heap.get(&(mbox + 1 + (i+1)*2)).copied().unwrap_or(0);
                        ctx.heap.insert(mbox + 1 + i*2, v);
                        i = i + 1;
                    }
                    ctx.heap.insert(mbox, count - 1);
                    return Ok(msg);
                }
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
        "aial_rt_actor_try_receive" => {
            let pid = args.first().copied().unwrap_or(0);
            let mbox = ctx.heap.get(&pid).copied().unwrap_or(0);
            let ptr = ctx.alloc();
            if mbox == 0 { ctx.string_store.insert(ptr, String::new()); return Ok(ptr); }
            let count = ctx.heap.get(&mbox).copied().unwrap_or(0);
            if count == 0 { ctx.string_store.insert(ptr, String::new()); return Ok(ptr); }
            let msg = ctx.heap.get(&(mbox + 1)).copied().unwrap_or(0);
            let mut i = 0;
            while i < count - 1 {
                let v = ctx.heap.get(&(mbox + 1 + (i+1)*2)).copied().unwrap_or(0);
                ctx.heap.insert(mbox + 1 + i*2, v);
                i = i + 1;
            }
            ctx.heap.insert(mbox, count - 1);
            Ok(msg)
        }
        "aial_rt_actor_recv_timeout" => {
            let pid = args.first().copied().unwrap_or(0);
            let mbox = ctx.heap.get(&pid).copied().unwrap_or(0);
            let ptr = ctx.alloc();
            if mbox == 0 { ctx.string_store.insert(ptr, String::new()); return Ok(ptr); }
            let count = ctx.heap.get(&mbox).copied().unwrap_or(0);
            if count == 0 { ctx.string_store.insert(ptr, String::new()); return Ok(ptr); }
            let msg = ctx.heap.get(&(mbox + 1)).copied().unwrap_or(0);
            let mut i = 0;
            while i < count - 1 {
                let v = ctx.heap.get(&(mbox + 1 + (i+1)*2)).copied().unwrap_or(0);
                ctx.heap.insert(mbox + 1 + i*2, v);
                i = i + 1;
            }
            ctx.heap.insert(mbox, count - 1);
            Ok(msg)
        }
        "aial_rt_actor_error" => {
            let ptr = ctx.alloc();
            ctx.string_store.insert(ptr, String::new());
            Ok(ptr)
        }
        _ => Err(format!("unknown runtime function: {}", name)),
    }
}

/// Write a serde_json::Value into the interpreter heap as a JsonValue structure.
/// Layout: [type(tag), bool_value, f64_bits_lo, array/object_handle, _]
/// Types: 0=null, 1=bool, 2=number, 3=string, 4=array, 5=object
fn write_json_to_heap(ctx: &mut EvalContext, ptr: i64, v: &serde_json::Value) {
    match v {
        serde_json::Value::Null => { ctx.heap.insert(ptr, 0); }
        serde_json::Value::Bool(b) => { ctx.heap.insert(ptr, 1); ctx.heap.insert(ptr + 1, *b as i64); }
        serde_json::Value::Number(n) => {
            ctx.heap.insert(ptr, 2);
            if let Some(f) = n.as_f64() { ctx.heap.insert(ptr + 2, f.to_bits() as i64); }
        }
        serde_json::Value::String(s) => {
            ctx.heap.insert(ptr, 3);
            let s_ptr = ctx.alloc();
            ctx.string_store.insert(s_ptr, s.clone());
            ctx.heap.insert(ptr + 1, s_ptr);
        }
        serde_json::Value::Array(arr) => {
            ctx.heap.insert(ptr, 4);
            let arr_ptr = ctx.alloc_block(arr.len());
            ctx.heap.insert(ptr + 3, arr_ptr);
            ctx.heap.insert(ptr + 2, arr.len() as i64);
            for (i, item) in arr.iter().enumerate() {
                let item_ptr = ctx.alloc_block(5);
                write_json_to_heap(ctx, item_ptr, item);
                ctx.heap.insert(arr_ptr + i as i64, item_ptr);
            }
        }
        serde_json::Value::Object(obj) => {
            ctx.heap.insert(ptr, 5);
            let n = obj.len();
            let obj_ptr = ctx.alloc_block(n * 2);
            ctx.heap.insert(ptr + 3, obj_ptr);
            ctx.heap.insert(ptr + 2, n as i64);
            for (i, (k, val)) in obj.iter().enumerate() {
                let k_ptr = ctx.alloc();
                ctx.string_store.insert(k_ptr, k.clone());
                ctx.heap.insert(obj_ptr + (i * 2) as i64, k_ptr);
                let v_ptr = ctx.alloc_block(5);
                write_json_to_heap(ctx, v_ptr, val);
                ctx.heap.insert(obj_ptr + (i * 2 + 1) as i64, v_ptr);
            }
        }
    }
}

/// Look up a key in a JsonValue object, returning a pointer to the value (or None)
fn json_lookup(ctx: &EvalContext, val_ptr: i64, key: &str) -> Option<i64> {
    let tag = ctx.heap.get(&val_ptr).copied().unwrap_or(0);
    if tag != 5 { return None; } // not an object
    let n = ctx.heap.get(&(val_ptr + 2)).copied().unwrap_or(0) as usize;
    let obj_ptr = ctx.heap.get(&(val_ptr + 3)).copied().unwrap_or(0);
    for i in 0..n {
        let k_ptr = ctx.heap.get(&(obj_ptr + (i * 2) as i64)).copied().unwrap_or(0);
        let k = lookup_string(ctx, k_ptr as usize);
        if k == key {
            return ctx.heap.get(&(obj_ptr + (i * 2 + 1) as i64)).copied();
        }
    }
    None
}

/// Convert a JsonValue in the heap to a JSON string (for stringify)
fn json_value_to_string(ctx: &EvalContext, val_ptr: i64) -> String {
    let tag = ctx.heap.get(&val_ptr).copied().unwrap_or(0);
    match tag {
        0 => "null".to_string(),
        1 => (ctx.heap.get(&(val_ptr + 1)).copied().unwrap_or(0) != 0).to_string(),
        2 => f64::from_bits(ctx.heap.get(&(val_ptr + 2)).copied().unwrap_or(0) as u64).to_string(),
        3 => format!("\"{}\"", lookup_string(ctx, ctx.heap.get(&(val_ptr + 1)).copied().unwrap_or(0) as usize)),
        4 => {
            let n = ctx.heap.get(&(val_ptr + 2)).copied().unwrap_or(0) as usize;
            let arr = ctx.heap.get(&(val_ptr + 3)).copied().unwrap_or(0);
            let mut items = Vec::new();
            for i in 0..n {
                if let Some(ip) = ctx.heap.get(&(arr + i as i64)).copied() {
                    items.push(json_value_to_string(ctx, ip));
                }
            }
            format!("[{}]", items.join(","))
        }
        5 => {
            let n = ctx.heap.get(&(val_ptr + 2)).copied().unwrap_or(0) as usize;
            let obj = ctx.heap.get(&(val_ptr + 3)).copied().unwrap_or(0);
            let mut pairs = Vec::new();
            for i in 0..n {
                let kp = ctx.heap.get(&(obj + (i * 2) as i64)).copied().unwrap_or(0);
                let vp = ctx.heap.get(&(obj + (i * 2 + 1) as i64)).copied().unwrap_or(0);
                let key = lookup_string(ctx, kp as usize);
                pairs.push(format!("\"{}\":{}", key, json_value_to_string(ctx, vp)));
            }
            format!("{{{}}}", pairs.join(","))
        }
        _ => "null".to_string(),
    }
}

fn call_ai_api(provider: &str, model: &str, api_key: &str, prompt: &str, temperature: f64, max_tokens: i64, format_code: i64) -> Result<String, String> {
    let api_url = match provider {
        "openai" => std::env::var("AIAL_API_URL").unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".to_string()),
        "deepseek" => "https://api.deepseek.com/v1/chat/completions".to_string(),
        "anthropic" => return Err("Anthropic API not yet supported".to_string()),
        _ => return Err(format!("unknown AI provider: {}", provider)),
    };

    let mut body = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": temperature,
        "max_tokens": max_tokens,
    });
    if format_code == 1 {
        body["response_format"] = serde_json::json!({"type": "json_object"});
    }

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| format!("failed to create HTTP client: {}", e))?;

    let resp = client.post(&api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    let status = resp.status();
    let resp_text = resp.text().map_err(|e| format!("failed to read response: {}", e))?;

    if !status.is_success() {
        return Err(format!("API returned error ({}): {}", status, resp_text));
    }

    let json: serde_json::Value = serde_json::from_str(&resp_text)
        .map_err(|e| format!("failed to parse JSON: {}", e))?;

    json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| "API response missing choices[0].message.content".to_string())
        .map(|s| s.to_string())
}
