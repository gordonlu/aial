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
        Intrinsic::ActorSend => "aial_rt_actor_send",
        Intrinsic::ActorReceive => "aial_rt_actor_receive",
        Intrinsic::Println => "aial_rt_println",
        Intrinsic::PrivacySensitive => "aial_rt_privacy_sensitive",
        Intrinsic::ContextForget => "aial_rt_ctx_forget",
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
        "aial_rt_cap_check" => Ok(1),
        "aial_rt_actor_spawn" => Ok(0),
        "aial_rt_actor_send" => Ok(0),
        "aial_rt_actor_receive" => Ok(0),
        _ => Err(format!("unknown runtime function: {}", name)),
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
