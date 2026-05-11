// ir_lower.rs
use crate::ir::*;

/// 运行时函数的元数据
#[derive(Clone)]
pub struct RuntimeFunction {
    pub name: String,
    pub params: Vec<IRType>,
    pub ret: IRType,
}

/// 存放所有需要的运行时函数
#[derive(Default)]
pub struct RuntimeRegistry {
    pub functions: Vec<RuntimeFunction>,
}

impl RuntimeRegistry {
    fn add(&mut self, name: &str, params: Vec<IRType>, ret: IRType) {
        if !self.functions.iter().any(|f| f.name == name) {
            self.functions.push(RuntimeFunction {
                name: name.to_string(),
                params,
                ret,
            });
        }
    }
}

/// 降低整个模块
pub fn lower_module(module: &IRModule) -> (IRModule, RuntimeRegistry) {
    let mut reg = RuntimeRegistry::default();
    let mut new_functions = Vec::new();

    for func in &module.functions {
        let lowered = lower_function(func, &mut reg);
        new_functions.push(lowered);
    }

    let new_module = IRModule {
        functions: new_functions,
        strings: module.strings.clone(),
        tool_registrations: module.tool_registrations.clone(),
    };

    (new_module, reg)
}

fn lower_function(func: &IRFunction, reg: &mut RuntimeRegistry) -> IRFunction {
    // Sort blocks by emission order (first Value ID) because creation order
    // can differ from emission order with nested control flow (if in while etc.)
    let mut sorted_indices: Vec<usize> = (0..func.blocks.len()).collect();
    sorted_indices.sort_by_key(|&i| {
        func.blocks[i].instrs.first()
            .and_then(|(_, v)| v.map(|vv| vv.0))
            .unwrap_or(u32::MAX)
    });

    let mut new_blocks: Vec<BasicBlock> = Vec::with_capacity(func.blocks.len());
    let mut vi = 0;
    for &idx in &sorted_indices {
        let block = &func.blocks[idx];
        let mut new_instrs = Vec::new();
        for (instr, _) in &block.instrs {
            let lowered = lower_instr(instr, reg);
            for li in lowered {
                let val = if vi < func.value_types.len() {
                    let v = func.value_types[vi].0;
                    vi += 1;
                    Some(v)
                } else {
                    None
                };
                new_instrs.push((li, val));
            }
        }
        new_blocks.push(BasicBlock {
            id: block.id,
            instrs: new_instrs,
            terminator: block.terminator.clone(),
        });
    }

    IRFunction {
        name: func.name.clone(),
        params: func.params.clone(),
        return_type: func.return_type.clone(),
        blocks: new_blocks,
        entry: func.entry,
        value_types: func.value_types.clone(),
    }
}

fn lower_instr(instr: &Instr, reg: &mut RuntimeRegistry) -> Vec<Instr> {
    match instr {
        Instr::IntrinsicCall { intrinsic, args, ret_ty: _ } => {
            let (fn_name, _fn_params, fn_ret) = match intrinsic {
                Intrinsic::AiCall => {
                    let params = vec![IRType::I64, IRType::I64, IRType::String, IRType::F64, IRType::I64, IRType::I64];
                    reg.add("aial_rt_ai_call", params.clone(),
                            IRType::AiResponse(Box::new(IRType::String)));
                    ("aial_rt_ai_call".to_string(), params,
                     IRType::AiResponse(Box::new(IRType::String)))
                },
                Intrinsic::AiCallMany => {
                    reg.add("aial_rt_ai_call_many", vec![], IRType::AiManyResponse(Box::new(IRType::String)));
                    ("aial_rt_ai_call_many".to_string(), vec![], IRType::AiManyResponse(Box::new(IRType::String)))
                },
                Intrinsic::AiCallRace => {
                    reg.add("aial_rt_ai_call_race", vec![], IRType::AiRaceResponse(Box::new(IRType::String)));
                    ("aial_rt_ai_call_race".to_string(), vec![], IRType::AiRaceResponse(Box::new(IRType::String)))
                },
                Intrinsic::ContextNew => {
                    let params = vec![IRType::String, IRType::I64, IRType::String, IRType::I64];
                    reg.add("aial_rt_ctx_new", params.clone(), IRType::I64);
                    ("aial_rt_ctx_new".to_string(), params, IRType::I64)
                },
                Intrinsic::ContextCurrent => {
                    reg.add("aial_rt_ctx_current", vec![], IRType::I64);
                    ("aial_rt_ctx_current".to_string(), vec![], IRType::I64)
                },
                Intrinsic::ContextBudget => {
                    reg.add("aial_rt_ctx_budget", vec![IRType::I64], IRType::I64);
                    ("aial_rt_ctx_budget".to_string(), vec![IRType::I64], IRType::I64)
                },
                Intrinsic::ExtractAiText => {
                    reg.add("aial_rt_extract_ai_text", vec![IRType::I64], IRType::String);
                    ("aial_rt_extract_ai_text".to_string(), vec![IRType::I64], IRType::String)
                },
                Intrinsic::ExtractAiVariant => {
                    reg.add("aial_rt_extract_ai_variant", vec![IRType::I64], IRType::I32);
                    ("aial_rt_extract_ai_variant".to_string(), vec![IRType::I64], IRType::I32)
                },
                Intrinsic::ExtractAiUsage => {
                    reg.add("aial_rt_extract_ai_usage", vec![IRType::I64], IRType::I64);
                    ("aial_rt_extract_ai_usage".to_string(), vec![IRType::I64], IRType::I64)
                },
                Intrinsic::ExtractAiReasoning => {
                    reg.add("aial_rt_extract_ai_reasoning", vec![IRType::I64], IRType::String);
                    ("aial_rt_extract_ai_reasoning".to_string(), vec![IRType::I64], IRType::String)
                },
                Intrinsic::ToolDispatch => {
                    reg.add("aial_rt_tool_dispatch", vec![IRType::String, IRType::String], IRType::String);
                    ("aial_rt_tool_dispatch".to_string(), vec![IRType::String, IRType::String], IRType::String)
                },
                Intrinsic::CapCheck => {
                    reg.add("aial_rt_cap_check", vec![IRType::String], IRType::Bool);
                    ("aial_rt_cap_check".to_string(), vec![IRType::String], IRType::Bool)
                },
                Intrinsic::ActorSpawn => {
                    reg.add("aial_rt_actor_spawn", vec![IRType::I64, IRType::I64], IRType::I64);
                    ("aial_rt_actor_spawn".to_string(), vec![IRType::I64, IRType::I64], IRType::I64)
                },
                Intrinsic::ActorSend => {
                    reg.add("aial_rt_actor_send", vec![IRType::I64, IRType::I64], IRType::Void);
                    ("aial_rt_actor_send".to_string(), vec![IRType::I64, IRType::I64], IRType::Void)
                },
                Intrinsic::ActorReceive => {
                    reg.add("aial_rt_actor_receive", vec![], IRType::I64);
                    ("aial_rt_actor_receive".to_string(), vec![], IRType::I64)
                },
                Intrinsic::Println => {
                    reg.add("aial_rt_println", vec![IRType::String], IRType::Void);
                    ("aial_rt_println".to_string(), vec![IRType::String], IRType::Void)
                },
                Intrinsic::PrivacySensitive => {
                    reg.add("aial_rt_privacy_sensitive", vec![IRType::I64], IRType::I64);
                    ("aial_rt_privacy_sensitive".to_string(), vec![IRType::I64], IRType::I64)
                },
                Intrinsic::ContextForget => {
                    reg.add("aial_rt_ctx_forget", vec![IRType::I64, IRType::I64], IRType::Void);
                    ("aial_rt_ctx_forget".to_string(), vec![IRType::I64, IRType::I64], IRType::Void)
                },
                Intrinsic::ContextReflect => {
                    reg.add("aial_rt_ctx_reflect", vec![IRType::I64], IRType::I64);
                    ("aial_rt_ctx_reflect".to_string(), vec![IRType::I64], IRType::I64)
                },
                Intrinsic::StrLen => {
                    reg.add("aial_rt_strlen", vec![IRType::String], IRType::I64);
                    ("aial_rt_strlen".to_string(), vec![IRType::String], IRType::I64)
                },
                Intrinsic::StrConcat => {
                    reg.add("aial_rt_strcat", vec![IRType::String, IRType::String], IRType::String);
                    ("aial_rt_strcat".to_string(), vec![IRType::String, IRType::String], IRType::String)
                },
                Intrinsic::StrSlice => {
                    reg.add("aial_rt_strslice", vec![IRType::String, IRType::I64, IRType::I64], IRType::String);
                    ("aial_rt_strslice".to_string(), vec![IRType::String, IRType::I64, IRType::I64], IRType::String)
                },
                Intrinsic::StrChr => {
                    reg.add("aial_rt_strchr", vec![IRType::String, IRType::I64], IRType::I64);
                    ("aial_rt_strchr".to_string(), vec![IRType::String, IRType::I64], IRType::I64)
                },
                Intrinsic::StrEq => {
                    reg.add("aial_rt_str_eq", vec![IRType::String, IRType::String], IRType::Bool);
                    ("aial_rt_str_eq".to_string(), vec![IRType::String, IRType::String], IRType::Bool)
                },
                Intrinsic::StartsWith => {
                    reg.add("aial_rt_starts_with", vec![IRType::String, IRType::String], IRType::Bool);
                    ("aial_rt_starts_with".to_string(), vec![IRType::String, IRType::String], IRType::Bool)
                },
                Intrinsic::FileRead => {
                    reg.add("aial_rt_file_read", vec![IRType::String], IRType::String);
                    ("aial_rt_file_read".to_string(), vec![IRType::String], IRType::String)
                },
                Intrinsic::FileWrite => {
                    reg.add("aial_rt_file_write", vec![IRType::String, IRType::String], IRType::Void);
                    ("aial_rt_file_write".to_string(), vec![IRType::String, IRType::String], IRType::Void)
                },
                Intrinsic::FileAppend => {
                    reg.add("aial_rt_file_append", vec![IRType::String, IRType::String], IRType::Void);
                    ("aial_rt_file_append".to_string(), vec![IRType::String, IRType::String], IRType::Void)
                },
                Intrinsic::EnumCreate => {
                    let n = args.len();
                    let mut params = vec![IRType::String, IRType::String];
                    for _ in 2..n { params.push(IRType::I64); }
                    reg.add("aial_rt_enum_create", params.clone(), IRType::I64);
                    ("aial_rt_enum_create".to_string(), params, IRType::I64)
                },
                Intrinsic::FilePatch => {
                    reg.add("aial_rt_file_patch", vec![IRType::String, IRType::String, IRType::String], IRType::Void);
                    ("aial_rt_file_patch".to_string(), vec![IRType::String, IRType::String, IRType::String], IRType::Void)
                },
                Intrinsic::HttpGet => {
                    reg.add("aial_rt_http_get", vec![IRType::String], IRType::HttpResponse);
                    ("aial_rt_http_get".to_string(), vec![IRType::String], IRType::HttpResponse)
                },
                Intrinsic::HttpStatus => {
                    reg.add("aial_rt_http_status", vec![IRType::HttpResponse], IRType::I64);
                    ("aial_rt_http_status".to_string(), vec![IRType::HttpResponse], IRType::I64)
                },
                Intrinsic::HttpText => {
                    reg.add("aial_rt_http_text", vec![IRType::HttpResponse], IRType::String);
                    ("aial_rt_http_text".to_string(), vec![IRType::HttpResponse], IRType::String)
                },
                Intrinsic::JsonParse => {
                    reg.add("aial_rt_json_parse", vec![IRType::String], IRType::JsonValue);
                    ("aial_rt_json_parse".to_string(), vec![IRType::String], IRType::JsonValue)
                },
                Intrinsic::JsonGet => {
                    reg.add("aial_rt_json_get", vec![IRType::JsonValue, IRType::String], IRType::JsonValue);
                    ("aial_rt_json_get".to_string(), vec![IRType::JsonValue, IRType::String], IRType::JsonValue)
                },
                Intrinsic::JsonGetOr => {
                    reg.add("aial_rt_json_get_or", vec![IRType::JsonValue, IRType::String, IRType::JsonValue], IRType::JsonValue);
                    ("aial_rt_json_get_or".to_string(), vec![IRType::JsonValue, IRType::String, IRType::JsonValue], IRType::JsonValue)
                },
                Intrinsic::JsonType => {
                    reg.add("aial_rt_json_type", vec![IRType::JsonValue], IRType::I64);
                    ("aial_rt_json_type".to_string(), vec![IRType::JsonValue], IRType::I64)
                },
                Intrinsic::JsonToString => {
                    reg.add("aial_rt_json_stringify", vec![IRType::JsonValue], IRType::String);
                    ("aial_rt_json_stringify".to_string(), vec![IRType::JsonValue], IRType::String)
                },
                Intrinsic::JsonValueToString => {
                    reg.add("aial_rt_json_value_to_string", vec![IRType::JsonValue], IRType::String);
                    ("aial_rt_json_value_to_string".to_string(), vec![IRType::JsonValue], IRType::String)
                },
                Intrinsic::JsonToInt => {
                    reg.add("aial_rt_json_to_int", vec![IRType::JsonValue], IRType::I64);
                    ("aial_rt_json_to_int".to_string(), vec![IRType::JsonValue], IRType::I64)
                },
                Intrinsic::JsonToFloat => {
                    reg.add("aial_rt_json_to_float", vec![IRType::JsonValue], IRType::F64);
                    ("aial_rt_json_to_float".to_string(), vec![IRType::JsonValue], IRType::F64)
                },
                Intrinsic::JsonArrayLen => {
                    reg.add("aial_rt_json_array_len", vec![IRType::JsonValue], IRType::I64);
                    ("aial_rt_json_array_len".to_string(), vec![IRType::JsonValue], IRType::I64)
                },
                Intrinsic::JsonArrayGet => {
                    reg.add("aial_rt_json_array_get", vec![IRType::JsonValue, IRType::I64], IRType::JsonValue);
                    ("aial_rt_json_array_get".to_string(), vec![IRType::JsonValue, IRType::I64], IRType::JsonValue)
                },
                Intrinsic::HttpPost => {
                    reg.add("aial_rt_http_post", vec![IRType::String, IRType::String], IRType::HttpResponse);
                    ("aial_rt_http_post".to_string(), vec![IRType::String, IRType::String], IRType::HttpResponse)
                },
                Intrinsic::HttpPostJson => {
                    reg.add("aial_rt_http_post_json", vec![IRType::String, IRType::JsonValue], IRType::HttpResponse);
                    ("aial_rt_http_post_json".to_string(), vec![IRType::String, IRType::JsonValue], IRType::HttpResponse)
                },
                Intrinsic::HttpHeaderMap => {
                    reg.add("aial_rt_http_header_map", vec![], IRType::I64);
                    ("aial_rt_http_header_map".to_string(), vec![], IRType::I64)
                },
                Intrinsic::HttpHeaderSet => {
                    reg.add("aial_rt_http_header_set", vec![IRType::I64, IRType::String, IRType::String], IRType::I64);
                    ("aial_rt_http_header_set".to_string(), vec![IRType::I64, IRType::String, IRType::String], IRType::I64)
                },
                Intrinsic::HttpStart => {
                    reg.add("aial_rt_http_start", vec![IRType::I64], IRType::I64);
                    ("aial_rt_http_start".to_string(), vec![IRType::I64], IRType::I64)
                },
                Intrinsic::HttpListen => {
                    reg.add("aial_rt_http_listen", vec![IRType::I64, IRType::I64], IRType::I64);
                    ("aial_rt_http_listen".to_string(), vec![IRType::I64, IRType::I64], IRType::I64)
                },
                Intrinsic::HttpRespond => {
                    reg.add("aial_rt_http_respond", vec![IRType::I64, IRType::String, IRType::String], IRType::Void);
                    ("aial_rt_http_respond".to_string(), vec![IRType::I64, IRType::String, IRType::String], IRType::Void)
                },
                Intrinsic::HttpRequestBody => {
                    reg.add("aial_rt_http_body", vec![IRType::I64], IRType::String);
                    ("aial_rt_http_body".to_string(), vec![IRType::I64], IRType::String)
                },
                Intrinsic::HtmlEscape => {
                    reg.add("aial_rt_html_escape", vec![IRType::String], IRType::String);
                    ("aial_rt_html_escape".to_string(), vec![IRType::String], IRType::String)
                },
                Intrinsic::AiStreamStart => {
                    let params = vec![IRType::I64, IRType::I64, IRType::String, IRType::F64, IRType::I64, IRType::I64];
                    reg.add("aial_rt_ai_stream_start", params.clone(), IRType::I64);
                    ("aial_rt_ai_stream_start".to_string(), params, IRType::I64)
                },
                Intrinsic::AiStreamRead => {
                    reg.add("aial_rt_ai_stream_read", vec![IRType::I64], IRType::String);
                    ("aial_rt_ai_stream_read".to_string(), vec![IRType::I64], IRType::String)
                },
                Intrinsic::IoReadln => {
                    reg.add("aial_rt_io_readln", vec![], IRType::String);
                    ("aial_rt_io_readln".to_string(), vec![], IRType::String)
                },
                Intrinsic::IoReadlnTimeout => {
                    reg.add("aial_rt_io_readln_timeout", vec![IRType::I64], IRType::String);
                    ("aial_rt_io_readln_timeout".to_string(), vec![IRType::I64], IRType::String)
                },
                Intrinsic::IoReadkey => {
                    reg.add("aial_rt_io_readkey", vec![], IRType::String);
                    ("aial_rt_io_readkey".to_string(), vec![], IRType::String)
                },
                Intrinsic::IoRawMode => {
                    reg.add("aial_rt_io_raw_mode", vec![IRType::I64], IRType::Void);
                    ("aial_rt_io_raw_mode".to_string(), vec![IRType::I64], IRType::Void)
                },
                Intrinsic::Print => {
                    reg.add("aial_rt_print", vec![IRType::String], IRType::Void);
                    ("aial_rt_print".to_string(), vec![IRType::String], IRType::Void)
                },
                Intrinsic::CtxOpenMemory => {
                    reg.add("aial_rt_ctx_open_memory", vec![IRType::String], IRType::I64);
                    ("aial_rt_ctx_open_memory".to_string(), vec![IRType::String], IRType::I64)
                },
                Intrinsic::CtxSaveMessage => {
                    reg.add("aial_rt_ctx_save_message", vec![IRType::I64, IRType::String, IRType::String, IRType::String], IRType::Void);
                    ("aial_rt_ctx_save_message".to_string(), vec![IRType::I64, IRType::String, IRType::String, IRType::String], IRType::Void)
                },
                Intrinsic::CtxLoadMessages => {
                    reg.add("aial_rt_ctx_load_messages", vec![IRType::I64, IRType::String, IRType::I64], IRType::String);
                    ("aial_rt_ctx_load_messages".to_string(), vec![IRType::I64, IRType::String, IRType::I64], IRType::String)
                },
                Intrinsic::CtxLoadMessagesSince => {
                    reg.add("aial_rt_ctx_load_messages_since", vec![IRType::I64, IRType::String, IRType::I64], IRType::String);
                    ("aial_rt_ctx_load_messages_since".to_string(), vec![IRType::I64, IRType::String, IRType::I64], IRType::String)
                },
                Intrinsic::CtxCloseMemory => {
                    reg.add("aial_rt_ctx_close_memory", vec![IRType::I64], IRType::Void);
                    ("aial_rt_ctx_close_memory".to_string(), vec![IRType::I64], IRType::Void)
                },
                Intrinsic::TimeSleep => {
                    reg.add("aial_rt_time_sleep", vec![IRType::I64], IRType::Void);
                    ("aial_rt_time_sleep".to_string(), vec![IRType::I64], IRType::Void)
                },
                Intrinsic::FfiLoad => {
                    reg.add("aial_rt_ffi_load", vec![IRType::String], IRType::I64);
                    ("aial_rt_ffi_load".to_string(), vec![IRType::String], IRType::I64)
                },
                Intrinsic::FfiCall => {
                    // Variadic: handle + name + up to 6 args
                    let mut params = vec![IRType::I64, IRType::String];
                    for _ in 2..args.len() { params.push(IRType::I64); }
                    reg.add("aial_rt_ffi_call", params.clone(), IRType::I64);
                    ("aial_rt_ffi_call".to_string(), params, IRType::I64)
                },
                Intrinsic::FfiClose => {
                    reg.add("aial_rt_ffi_close", vec![IRType::I64], IRType::Void);
                    ("aial_rt_ffi_close".to_string(), vec![IRType::I64], IRType::Void)
                },
            };

            vec![Instr::ExternCall {
                name: fn_name,
                args: args.clone(),
                ret_ty: fn_ret,
            }]
        },
        other => vec![other.clone()],
    }
}
