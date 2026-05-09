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
    let mut new_blocks = Vec::new();
    let mut vi = 0;
    for block in &func.blocks {
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
