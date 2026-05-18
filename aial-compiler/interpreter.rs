// interpreter.rs — AAL IR interpreter
// Replaces the deprecated cranelift-simplejit backend.
// Will be superseded by cranelift-jit or LLVM once the language stabilizes.
//
// Runtime semantics delegated to aial-rt — this shim manages control flow,
// value tracking, and bridges to the runtime for all runtime operations.

use crate::ir::*;
use std::collections::HashMap;
use aial_rt::lock;

pub fn interpret(module: &IRModule) -> Result<(), String> {
    // Register all compile-time string constants with the runtime so that
    // delegated aial_rt_* calls can look them up by index.
    for (i, s) in module.strings.iter().enumerate() {
        lock!(aial_rt::strs()).insert(i as i64, s.clone());
    }

    let main_fn = module
        .functions
        .iter()
        .find(|f| f.name == "main")
        .ok_or_else(|| "no main function found".to_string())?;

    let mut ctx = EvalContext::new(&module.strings, &module.tool_registrations);
    exec_func(&mut ctx, main_fn, &[], module)?;
    Ok(())
}

struct EvalContext<'a> {
    values: HashMap<Value, i64>,
    strings: &'a [String],
    tools: &'a [ToolRegistration],
    #[allow(dead_code)]
    tainted: std::collections::HashSet<i64>,
}

impl<'a> EvalContext<'a> {
    fn new(strings: &'a [String], tools: &'a [ToolRegistration]) -> Self {
        EvalContext {
            values: HashMap::new(),
            strings,
            tools,
            tainted: std::collections::HashSet::new(),
        }
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
            let result_val = instr_values
                .iter()
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
        Instr::ConstBool(b) => {
            if *b {
                1
            } else {
                0
            }
        }
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
                BinOp::Div => {
                    if rv == 0 {
                        return Err("division by zero".to_string());
                    }
                    lv.wrapping_div(rv)
                }
                BinOp::Rem => {
                    if rv == 0 {
                        return Err("division by zero".to_string());
                    }
                    lv.wrapping_rem(rv)
                }
                BinOp::Eq => (lv == rv) as i64,
                BinOp::Ne => (lv != rv) as i64,
                BinOp::Lt => (lv < rv) as i64,
                BinOp::Gt => (lv > rv) as i64,
                BinOp::Le => (lv <= rv) as i64,
                BinOp::Ge => (lv >= rv) as i64,
                BinOp::And => {
                    if lv != 0 && rv != 0 {
                        1
                    } else {
                        0
                    }
                }
                BinOp::Or => {
                    if lv != 0 || rv != 0 {
                        1
                    } else {
                        0
                    }
                }
            }
        }
        Instr::UnOp(op, operand) => {
            let v = *ctx.values.get(operand).unwrap_or(&0);
            use crate::ast::UnOp;
            match op {
                UnOp::Neg => v.wrapping_neg(),
                UnOp::Not => (v == 0) as i64,
            }
        }
        Instr::Cmp(op, l, r) => {
            let lv = *ctx.values.get(l).unwrap_or(&0);
            let rv = *ctx.values.get(r).unwrap_or(&0);
            use crate::ast::BinOp;
            match op {
                BinOp::Eq => (lv == rv) as i64,
                BinOp::Ne => (lv != rv) as i64,
                BinOp::Lt => (lv < rv) as i64,
                BinOp::Gt => (lv > rv) as i64,
                BinOp::Le => (lv <= rv) as i64,
                BinOp::Ge => (lv >= rv) as i64,
                _ => 0,
            }
        }
        Instr::Alloca(_ty) => aial_rt::alloc(),
        Instr::Load(ptr) => {
            let addr = *ctx.values.get(ptr).unwrap_or(&0);
            lock!(aial_rt::heap())
                .get(&addr)
                .copied()
                .unwrap_or(0)
        }
        Instr::Store(ptr, val) => {
            let addr = *ctx.values.get(ptr).unwrap_or(&0);
            let v = *ctx.values.get(val).unwrap_or(&0);
            lock!(aial_rt::heap()).insert(addr, v);
            0
        }
        Instr::ExtractValue {
            aggregate,
            index,
        } => {
            let base = *ctx.values.get(aggregate).unwrap_or(&0);
            lock!(aial_rt::heap())
                .get(&(base + *index as i64))
                .copied()
                .unwrap_or(0)
        }
        Instr::InsertValue {
            aggregate,
            element,
            index,
        } => {
            let base = *ctx.values.get(aggregate).unwrap_or(&0);
            let v = *ctx.values.get(element).unwrap_or(&0);
            lock!(aial_rt::heap()).insert(base + *index as i64, v);
            0
        }
        Instr::Call { args, .. } => {
            let _ = args;
            0
        }
        Instr::UserCall {
            name, args, ..
        } => {
            let a: Vec<i64> = args
                .iter()
                .map(|v| *ctx.values.get(v).unwrap_or(&0))
                .collect();
            let func = module.functions.iter().find(|f| f.name == name.as_str());
            match func {
                Some(f) => match exec_func(ctx, f, &a, module)? {
                    Some(v) => v,
                    None => 0,
                },
                None => 0,
            }
        }
        Instr::IntrinsicCall {
            intrinsic, args, ..
        } => {
            let a: Vec<i64> = args
                .iter()
                .map(|v| *ctx.values.get(v).unwrap_or(&0))
                .collect();
            handle_runtime_call(ctx, intrinsic_to_name(intrinsic), &a, module)?
        }
        Instr::ExternCall { name, args, .. } => {
            let a: Vec<i64> = args
                .iter()
                .map(|v| *ctx.values.get(v).unwrap_or(&0))
                .collect();
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
        Intrinsic::StrPrevChar => "aial_rt_str_prev_char",
        Intrinsic::StrNextChar => "aial_rt_str_next_char",
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
        Intrinsic::IoReadMultiline => "aial_rt_io_read_multiline",
        Intrinsic::IoReadkeyTimeout => "aial_rt_io_readkey_timeout",
        Intrinsic::IoRawMode => "aial_rt_io_raw_mode",
        Intrinsic::IoTty => "aial_rt_io_is_tty",
        Intrinsic::TermDisplayWidth => "aial_rt_term_display_width",
        Intrinsic::Print => "aial_rt_print",
        Intrinsic::CtxOpenMemory => "aial_rt_ctx_open_memory",
        Intrinsic::CtxSaveMessage => "aial_rt_ctx_save_message",
        Intrinsic::CtxLoadMessages => "aial_rt_ctx_load_messages",
        Intrinsic::CtxLoadMessagesSince => "aial_rt_ctx_load_messages_since",
        Intrinsic::CtxCloseMemory => "aial_rt_ctx_close_memory",
        Intrinsic::CtxLastError => "aial_rt_ctx_last_error",
        Intrinsic::LineNew => "aial_rt_line_new",
        Intrinsic::LineRead => "aial_rt_line_read",
        Intrinsic::LineRedraw => "aial_rt_line_redraw",
        Intrinsic::LineEnd => "aial_rt_line_end",
        Intrinsic::TermHeight => "aial_rt_term_height",
        Intrinsic::TermScroll => "aial_rt_term_scroll_region",
        Intrinsic::TermSetup => "aial_rt_term_setup",
        Intrinsic::TermRedraw => "aial_rt_term_redraw",
        Intrinsic::TermCursorGoto => "aial_rt_term_cursor_goto",
        Intrinsic::TermReset => "aial_rt_term_reset",
        Intrinsic::TermClear => "aial_rt_term_clear",
        Intrinsic::TimeNowMs => "aial_rt_time_now_ms",
        Intrinsic::ProcessRun => "aial_rt_process_run",
        Intrinsic::ProcessRunWithStatus => "aial_rt_process_run_with_status",
        Intrinsic::ProcessExec => "aial_rt_process_exec",
        Intrinsic::IntToString => "aial_rt_int_to_string",
        Intrinsic::StringToInt => "aial_rt_string_to_int",
        Intrinsic::Args => "aial_rt_args",
        Intrinsic::StrFind => "aial_rt_str_find",
        Intrinsic::FileListDir => "aial_rt_file_list_dir",
        Intrinsic::TermDrawClipped => "aial_rt_term_draw_text_clipped",
        Intrinsic::TermCursorRow => "aial_rt_term_cursor_row",
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
        Intrinsic::GlobalSet => "aial_rt_global_set",
        Intrinsic::GlobalGet => "aial_rt_global_get",
        Intrinsic::GlobalHas => "aial_rt_global_has",
        Intrinsic::GlobalDelete => "aial_rt_global_delete",
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
        Intrinsic::ArrayJoin => "aial_rt_array_join",
    }
}

fn handle_runtime_call(
    _ctx: &mut EvalContext,
    name: &str,
    args: &[i64],
    _module: &IRModule,
) -> Result<i64, String> {
    match name {
        // ── AI calls ──
        "aial_rt_ai_call" => {
            let model = args.first().copied().unwrap_or(0);
            let ctx_id = args.get(1).copied().unwrap_or(0);
            let prompt_idx = args.get(2).copied().unwrap_or(0);
            let temp = f64::from_bits(args.get(3).copied().unwrap_or(0) as u64);
            let max_tokens = args.get(4).copied().unwrap_or(1024);
            let format_code = args.get(5).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_ai_call(
                model,
                ctx_id,
                prompt_idx,
                temp,
                max_tokens,
                format_code,
            ))
        }
        "aial_rt_ai_call_many" => {
            Ok(aial_rt::aial_rt_ai_call_many(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_ai_call_race" => {
            Ok(aial_rt::aial_rt_ai_call_race(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_ai_call_raw" => {
            let model = args.first().copied().unwrap_or(0);
            let prompt_ptr = args.get(1).copied().unwrap_or(0);
            let max_tokens = args.get(2).copied().unwrap_or(256);
            Ok(aial_rt::aial_rt_ai_call_raw(
                model, prompt_ptr, max_tokens,
            ))
        }

        // ── AI streaming ──
        "aial_rt_ai_stream_start" => {
            let model = args.first().copied().unwrap_or(0);
            let ctx_id = args.get(1).copied().unwrap_or(0);
            let prompt_idx = args.get(2).copied().unwrap_or(0);
            let temp = f64::from_bits(args.get(3).copied().unwrap_or(0) as u64);
            let max_tokens = args.get(4).copied().unwrap_or(1024);
            let format_code = args.get(5).copied().unwrap_or(0);
            let tools_json = args.get(6).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_ai_stream_start(
                model, ctx_id, prompt_idx, temp, max_tokens, format_code, tools_json,
            ))
        }
        "aial_rt_ai_stream_read" => {
            Ok(aial_rt::aial_rt_ai_stream_read(args.first().copied().unwrap_or(0)))
        }

        // ── Context management ──
        "aial_rt_ctx_new" => {
            let prompt_ptr = args.first().copied().unwrap_or(0);
            let budget = args.get(1).copied().unwrap_or(4096);
            let strategy = args.get(2).copied().unwrap_or(0);
            let ws = args.get(3).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_ctx_new(prompt_ptr, budget, strategy, ws))
        }
        "aial_rt_ctx_current" => Ok(aial_rt::aial_rt_ctx_current()),
        "aial_rt_ctx_budget" => Ok(aial_rt::aial_rt_ctx_budget(args.first().copied().unwrap_or(0))),
        "aial_rt_ctx_forget" => {
            let ctx_id = args.first().copied().unwrap_or(0);
            let msg_id = args.get(1).copied().unwrap_or(0);
            aial_rt::aial_rt_ctx_forget(ctx_id, msg_id);
            Ok(0)
        }
        "aial_rt_ctx_add_message" => {
            let ctx_id = args.first().copied().unwrap_or(0);
            let role = args.get(1).copied().unwrap_or(0);
            let content = args.get(2).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_ctx_add_message(ctx_id, role, content))
        }
        "aial_rt_ctx_reflect" => {
            Ok(aial_rt::aial_rt_ctx_reflect(args.first().copied().unwrap_or(0)))
        }

        // ── Context memory (SQLite) ──
        "aial_rt_ctx_open_memory" => {
            Ok(aial_rt::aial_rt_ctx_open_memory(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_ctx_save_message" => {
            aial_rt::aial_rt_ctx_save_message(
                args.first().copied().unwrap_or(0),
                args.get(1).copied().unwrap_or(0),
                args.get(2).copied().unwrap_or(0),
                args.get(3).copied().unwrap_or(0),
            );
            Ok(0)
        }
        "aial_rt_ctx_load_messages" => {
            let db = args.first().copied().unwrap_or(0);
            let session = args.get(1).copied().unwrap_or(0);
            let limit = args.get(2).copied().unwrap_or(50);
            Ok(aial_rt::aial_rt_ctx_load_messages(db, session, limit))
        }
        "aial_rt_ctx_load_messages_since" => {
            let db = args.first().copied().unwrap_or(0);
            let session = args.get(1).copied().unwrap_or(0);
            let ts = args.get(2).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_ctx_load_messages_since(
                db, session, ts,
            ))
        }
        "aial_rt_ctx_close_memory" => {
            aial_rt::aial_rt_ctx_close_memory(args.first().copied().unwrap_or(0));
            Ok(0)
        }
        "aial_rt_ctx_last_error" => Ok(aial_rt::aial_rt_ctx_last_error()),

        // ── Extractors ──
        "aial_rt_extract_ai_text" => {
            Ok(aial_rt::aial_rt_extract_ai_text(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_extract_ai_variant" => {
            Ok(aial_rt::aial_rt_extract_ai_variant(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_extract_ai_usage" => {
            Ok(aial_rt::aial_rt_extract_ai_usage(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_extract_ai_reasoning" => {
            Ok(aial_rt::aial_rt_extract_ai_reasoning(args.first().copied().unwrap_or(0)))
        }

        // ── Capability & privacy ──
        "aial_rt_cap_check" => Ok(aial_rt::aial_rt_cap_check(args.first().copied().unwrap_or(0))),
        "aial_rt_privacy_sensitive" => {
            Ok(aial_rt::aial_rt_privacy_sensitive(args.first().copied().unwrap_or(0)))
        }

        // ── Tool dispatch ──
        "aial_rt_tool_dispatch" => Ok(aial_rt::aial_rt_tool_dispatch(
            args.first().copied().unwrap_or(0),
            args.get(1).copied().unwrap_or(0),
        )),

        // ── Print / I/O ──
        "aial_rt_println" => {
            aial_rt::aial_rt_println(args.first().copied().unwrap_or(0));
            Ok(0)
        }
        "aial_rt_print" => {
            aial_rt::aial_rt_print(args.first().copied().unwrap_or(0));
            Ok(0)
        }
        "aial_rt_io_readln" => Ok(aial_rt::aial_rt_io_readln()),
        "aial_rt_io_readln_timeout" => {
            Ok(aial_rt::aial_rt_io_readln_timeout(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_io_readkey" => Ok(aial_rt::aial_rt_io_readkey()),
        "aial_rt_io_readkey_timeout" => {
            Ok(aial_rt::aial_rt_io_readkey_timeout(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_io_read_multiline" => Ok(aial_rt::aial_rt_io_read_multiline()),
        "aial_rt_io_raw_mode" => {
            aial_rt::aial_rt_io_raw_mode(args.first().copied().unwrap_or(0));
            Ok(0)
        }
        "aial_rt_io_is_tty" => Ok(aial_rt::aial_rt_io_is_tty()),

        // ── Terminal ──
        "aial_rt_term_clear" => {
            aial_rt::aial_rt_term_clear();
            Ok(0)
        }
        "aial_rt_term_height" => Ok(aial_rt::aial_rt_term_height()),
        "aial_rt_term_scroll_region" => {
            aial_rt::aial_rt_term_scroll_region(
                args.first().copied().unwrap_or(0),
                args.get(1).copied().unwrap_or(0),
            );
            Ok(0)
        }
        "aial_rt_term_setup" => {
            aial_rt::aial_rt_term_setup(args.first().copied().unwrap_or(0));
            Ok(0)
        }
        "aial_rt_term_redraw" => {
            aial_rt::aial_rt_term_redraw(
                args.first().copied().unwrap_or(0),
                args.get(1).copied().unwrap_or(0),
                args.get(2).copied().unwrap_or(0),
                args.get(3).copied().unwrap_or(0),
            );
            Ok(0)
        }
        "aial_rt_term_cursor_goto" => {
            aial_rt::aial_rt_term_cursor_goto(
                args.first().copied().unwrap_or(0),
                args.get(1).copied().unwrap_or(0),
            );
            Ok(0)
        }
        "aial_rt_term_reset" => {
            aial_rt::aial_rt_term_reset();
            Ok(0)
        }
        "aial_rt_term_draw_text_clipped" => {
            aial_rt::aial_rt_term_draw_text_clipped(
                args.first().copied().unwrap_or(0),
                args.get(1).copied().unwrap_or(0),
                args.get(2).copied().unwrap_or(0),
                args.get(3).copied().unwrap_or(0),
            );
            Ok(0)
        }
        "aial_rt_term_cursor_row" => Ok(aial_rt::aial_rt_term_cursor_row()),
        "aial_rt_term_display_width" => {
            Ok(aial_rt::aial_rt_term_display_width(args.first().copied().unwrap_or(0)))
        }

        // ── Line editor ──
        "aial_rt_line_new" => Ok(aial_rt::aial_rt_line_new(args.first().copied().unwrap_or(0))),
        "aial_rt_line_read" => Ok(aial_rt::aial_rt_line_read(args.first().copied().unwrap_or(0))),
        "aial_rt_line_redraw" => {
            aial_rt::aial_rt_line_redraw(args.first().copied().unwrap_or(0));
            Ok(0)
        }
        "aial_rt_line_end" => {
            aial_rt::aial_rt_line_end(args.first().copied().unwrap_or(0));
            Ok(0)
        }

        // ── String operations ──
        "aial_rt_strlen" => Ok(aial_rt::aial_rt_strlen(args.first().copied().unwrap_or(0))),
        "aial_rt_strcat" => {
            let a = args.first().copied().unwrap_or(0);
            let b = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_strcat(a, b))
        }
        "aial_rt_strslice" => {
            let ptr = args.first().copied().unwrap_or(0);
            let start = args.get(1).copied().unwrap_or(0);
            let len = args.get(2).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_strslice(ptr, start, len))
        }
        "aial_rt_strchr" => {
            let ptr = args.first().copied().unwrap_or(0);
            let idx = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_strchr(ptr, idx))
        }
        "aial_rt_str_prev_char" => {
            let s = args.first().copied().unwrap_or(0);
            let pos = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_str_prev_char(s, pos))
        }
        "aial_rt_str_next_char" => {
            let s = args.first().copied().unwrap_or(0);
            let pos = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_str_next_char(s, pos))
        }
        "aial_rt_str_eq" => {
            let a = args.first().copied().unwrap_or(0);
            let b = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_str_eq(a, b))
        }
        "aial_rt_starts_with" => {
            let s = args.first().copied().unwrap_or(0);
            let pre = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_starts_with(s, pre))
        }
        "aial_rt_str_find" => {
            let haystack = args.first().copied().unwrap_or(0);
            let needle = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_str_find(haystack, needle))
        }
        "aial_rt_string_to_int" => {
            Ok(aial_rt::aial_rt_string_to_int(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_int_to_string" => {
            Ok(aial_rt::aial_rt_int_to_string(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_token_estimate" => {
            Ok(aial_rt::aial_rt_token_estimate(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_html_escape" => {
            Ok(aial_rt::aial_rt_html_escape(args.first().copied().unwrap_or(0)))
        }

        // ── File operations ──
        "aial_rt_file_read" => Ok(aial_rt::aial_rt_file_read(args.first().copied().unwrap_or(0))),
        "aial_rt_file_write" => {
            aial_rt::aial_rt_file_write(args.first().copied().unwrap_or(0), args.get(1).copied().unwrap_or(0));
            Ok(0)
        }
        "aial_rt_file_append" => {
            aial_rt::aial_rt_file_append(args.first().copied().unwrap_or(0), args.get(1).copied().unwrap_or(0));
            Ok(0)
        }
        "aial_rt_file_patch" => {
            aial_rt::aial_rt_file_patch(
                args.first().copied().unwrap_or(0),
                args.get(1).copied().unwrap_or(0),
                args.get(2).copied().unwrap_or(0),
            );
            Ok(0)
        }
        "aial_rt_file_list_dir" => {
            Ok(aial_rt::aial_rt_file_list_dir(args.first().copied().unwrap_or(0)))
        }

        // ── Enum ──
        "aial_rt_enum_create" => {
            let name_ptr = args.first().copied().unwrap_or(0);
            let variant_ptr = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_enum_create(name_ptr, variant_ptr))
        }

        // ── HTTP client ──
        "aial_rt_http_get" => Ok(aial_rt::aial_rt_http_get(args.first().copied().unwrap_or(0))),
        "aial_rt_http_status" => Ok(aial_rt::aial_rt_http_status(args.first().copied().unwrap_or(0))),
        "aial_rt_http_text" => Ok(aial_rt::aial_rt_http_text(args.first().copied().unwrap_or(0))),
        "aial_rt_http_post" => {
            let url = args.first().copied().unwrap_or(0);
            let body = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_http_post(url, body))
        }
        "aial_rt_http_post_json" => {
            let url = args.first().copied().unwrap_or(0);
            let val = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_http_post_json(url, val))
        }
        "aial_rt_http_header_map" => Ok(aial_rt::aial_rt_http_header_map()),
        "aial_rt_http_header_set" => {
            let map = args.first().copied().unwrap_or(0);
            let key = args.get(1).copied().unwrap_or(0);
            let val = args.get(2).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_http_header_set(map, key, val))
        }

        // ── HTTP server ──
        "aial_rt_http_start" => Ok(aial_rt::aial_rt_http_start(args.first().copied().unwrap_or(0))),
        "aial_rt_http_listen" => {
            let handle = args.first().copied().unwrap_or(0);
            let timeout = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_http_listen(handle, timeout))
        }
        "aial_rt_http_respond" => {
            let req = args.first().copied().unwrap_or(0);
            let body_ptr = args.get(1).copied().unwrap_or(0);
            let ct_ptr = args.get(2).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_http_respond(req, body_ptr, ct_ptr))
        }
        "aial_rt_http_body" => Ok(aial_rt::aial_rt_http_body(args.first().copied().unwrap_or(0))),
        "aial_rt_http_method" => Ok(aial_rt::aial_rt_http_method(args.first().copied().unwrap_or(0))),
        "aial_rt_http_path" => Ok(aial_rt::aial_rt_http_path(args.first().copied().unwrap_or(0))),
        "aial_rt_http_query" => {
            let req = args.first().copied().unwrap_or(0);
            let key = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_http_query(req, key))
        }
        "aial_rt_http_header" => {
            let req = args.first().copied().unwrap_or(0);
            let key = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_http_header(req, key))
        }
        "aial_rt_http_url" => Ok(aial_rt::aial_rt_http_url(args.first().copied().unwrap_or(0))),
        "aial_rt_http_status_text" => {
            Ok(aial_rt::aial_rt_http_status_text(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_http_ok" => {
            aial_rt::aial_rt_http_ok(args.first().copied().unwrap_or(0), args.get(1).copied().unwrap_or(0));
            Ok(0)
        }
        "aial_rt_http_json" => {
            aial_rt::aial_rt_http_json(args.first().copied().unwrap_or(0), args.get(1).copied().unwrap_or(0));
            Ok(0)
        }
        "aial_rt_http_html" => {
            aial_rt::aial_rt_http_html(args.first().copied().unwrap_or(0), args.get(1).copied().unwrap_or(0));
            Ok(0)
        }
        "aial_rt_http_serve" => {
            aial_rt::aial_rt_http_serve(args.first().copied().unwrap_or(0), args.get(1).copied().unwrap_or(0));
            Ok(0)
        }

        // ── JSON ──
        "aial_rt_json_parse" => Ok(aial_rt::aial_rt_json_parse(args.first().copied().unwrap_or(0))),
        "aial_rt_json_get" => {
            let val = args.first().copied().unwrap_or(0);
            let key = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_json_get(val, key))
        }
        "aial_rt_json_get_or" => {
            let val = args.first().copied().unwrap_or(0);
            let key = args.get(1).copied().unwrap_or(0);
            let default = args.get(2).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_json_get_or(val, key, default))
        }
        "aial_rt_json_type" => Ok(aial_rt::aial_rt_json_type(args.first().copied().unwrap_or(0))),
        "aial_rt_json_stringify" => {
            Ok(aial_rt::aial_rt_json_stringify(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_json_value_to_string" => {
            Ok(aial_rt::aial_rt_json_value_to_string(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_json_to_int" => Ok(aial_rt::aial_rt_json_to_int(args.first().copied().unwrap_or(0))),
        "aial_rt_json_to_float" => {
            let f = aial_rt::aial_rt_json_to_float(args.first().copied().unwrap_or(0));
            Ok(f.to_bits() as i64)
        }
        "aial_rt_json_array_len" => {
            Ok(aial_rt::aial_rt_json_array_len(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_json_array_get" => {
            let val = args.first().copied().unwrap_or(0);
            let idx = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_json_array_get(val, idx))
        }

        // ── Time ──
        "aial_rt_time_now_ms" => Ok(aial_rt::aial_rt_time_now_ms()),
        "aial_rt_time_now" => Ok(aial_rt::aial_rt_time_now()),
        "aial_rt_time_sleep" => {
            aial_rt::aial_rt_time_sleep(args.first().copied().unwrap_or(0));
            Ok(0)
        }

        // ── Process ──
        "aial_rt_process_run" => Ok(aial_rt::aial_rt_process_run(args.first().copied().unwrap_or(0))),
        "aial_rt_process_run_with_status" => {
            Ok(aial_rt::aial_rt_process_run_with_status(args.first().copied().unwrap_or(0)))
        }

        // ── Arguments ──
        "aial_rt_args" => Ok(aial_rt::aial_rt_args()),

        // ── FFI ──
        "aial_rt_ffi_load" => Ok(aial_rt::aial_rt_ffi_load(args.first().copied().unwrap_or(0))),
        "aial_rt_ffi_call" => {
            let handle = args.first().copied().unwrap_or(0);
            let fn_name = args.get(1).copied().unwrap_or(0);
            let a1 = args.get(2).copied().unwrap_or(0);
            let a2 = args.get(3).copied().unwrap_or(0);
            let a3 = args.get(4).copied().unwrap_or(0);
            let a4 = args.get(5).copied().unwrap_or(0);
            let a5 = args.get(6).copied().unwrap_or(0);
            let a6 = args.get(7).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_ffi_call(handle, fn_name, a1, a2, a3, a4, a5, a6))
        }
        "aial_rt_ffi_close" => {
            aial_rt::aial_rt_ffi_close(args.first().copied().unwrap_or(0));
            Ok(0)
        }

        // ── Map ──
        "aial_rt_map_new" => Ok(aial_rt::aial_rt_map_new()),
        "aial_rt_map_set" => {
            aial_rt::aial_rt_map_set(
                args.first().copied().unwrap_or(0),
                args.get(1).copied().unwrap_or(0),
                args.get(2).copied().unwrap_or(0),
            );
            Ok(0)
        }
        "aial_rt_map_get" => {
            let handle = args.first().copied().unwrap_or(0);
            let key = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_map_get(handle, key))
        }
        "aial_rt_map_has" => {
            let handle = args.first().copied().unwrap_or(0);
            let key = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_map_has(handle, key))
        }
        "aial_rt_map_remove" => {
            aial_rt::aial_rt_map_remove(
                args.first().copied().unwrap_or(0),
                args.get(1).copied().unwrap_or(0),
            );
            Ok(0)
        }

        // ── Key management ──
        "aial_rt_key_set" => {
            Ok(aial_rt::aial_rt_key_set(
                args.first().copied().unwrap_or(0),
                args.get(1).copied().unwrap_or(0),
            ))
        }
        "aial_rt_key_exists" => Ok(aial_rt::aial_rt_key_exists(args.first().copied().unwrap_or(0))),
        "aial_rt_key_delete" => Ok(aial_rt::aial_rt_key_delete(args.first().copied().unwrap_or(0))),

        // ── Global storage ──
        "aial_rt_global_set" => {
            aial_rt::aial_rt_global_set(
                args.first().copied().unwrap_or(0),
                args.get(1).copied().unwrap_or(0),
            );
            Ok(0)
        }
        "aial_rt_global_get" => Ok(aial_rt::aial_rt_global_get(args.first().copied().unwrap_or(0))),
        "aial_rt_global_has" => Ok(aial_rt::aial_rt_global_has(args.first().copied().unwrap_or(0))),
        "aial_rt_global_delete" => {
            aial_rt::aial_rt_global_delete(args.first().copied().unwrap_or(0));
            Ok(0)
        }

        // ── Heap (priority queue) ──
        "aial_rt_heap_new" => Ok(aial_rt::aial_rt_heap_new()),
        "aial_rt_heap_push" => {
            aial_rt::aial_rt_heap_push(
                args.first().copied().unwrap_or(0),
                args.get(1).copied().unwrap_or(0),
                args.get(2).copied().unwrap_or(0),
            );
            Ok(0)
        }
        "aial_rt_heap_pop" => Ok(aial_rt::aial_rt_heap_pop(args.first().copied().unwrap_or(0))),
        "aial_rt_heap_peek" => Ok(aial_rt::aial_rt_heap_peek(args.first().copied().unwrap_or(0))),
        "aial_rt_heap_len" => Ok(aial_rt::aial_rt_heap_len(args.first().copied().unwrap_or(0))),

        // ── Array ──
        "aial_rt_array_new" => Ok(aial_rt::aial_rt_array_new()),
        "aial_rt_array_push" => {
            aial_rt::aial_rt_array_push(
                args.first().copied().unwrap_or(0),
                args.get(1).copied().unwrap_or(0),
            );
            Ok(0)
        }
        "aial_rt_array_sort" => {
            aial_rt::aial_rt_array_sort(args.first().copied().unwrap_or(0));
            Ok(0)
        }
        "aial_rt_array_get" => {
            let handle = args.first().copied().unwrap_or(0);
            let index = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_array_get(handle, index))
        }
        "aial_rt_array_len" => Ok(aial_rt::aial_rt_array_len(args.first().copied().unwrap_or(0))),
        "aial_rt_array_join" => {
            let handle = args.first().copied().unwrap_or(0);
            let sep = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_array_join(handle, sep))
        }

        // ── Actor ──
        "aial_rt_actor_spawn" => Ok(aial_rt::aial_rt_actor_spawn()),
        "aial_rt_actor_spawn_handler" => {
            Ok(aial_rt::aial_rt_actor_spawn_handler(
                args.first().copied().unwrap_or(0),
                args.get(1).copied().unwrap_or(0),
            ))
        }
        "aial_rt_actor_send" => {
            aial_rt::aial_rt_actor_send(args.first().copied().unwrap_or(0), args.get(1).copied().unwrap_or(0));
            Ok(0)
        }
        "aial_rt_actor_receive" => {
            Ok(aial_rt::aial_rt_actor_receive(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_actor_try_receive" => {
            Ok(aial_rt::aial_rt_actor_try_receive(args.first().copied().unwrap_or(0)))
        }
        "aial_rt_actor_recv_timeout" => {
            let pid = args.first().copied().unwrap_or(0);
            let timeout = args.get(1).copied().unwrap_or(0);
            Ok(aial_rt::aial_rt_actor_recv_timeout(pid, timeout))
        }
        "aial_rt_actor_error" => Ok(aial_rt::aial_rt_actor_error(args.first().copied().unwrap_or(0))),

        _ => Err(format!("unknown runtime function: {}", name)),
    }
}
