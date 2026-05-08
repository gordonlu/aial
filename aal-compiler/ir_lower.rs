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
    for block in &func.blocks {
        let mut new_instrs = Vec::new();
        for instr in &block.instrs {
            let lowered = lower_instr(instr, reg);
            new_instrs.extend(lowered);
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
                    let params = if args.len() > 5 {
                        vec![IRType::I64, IRType::I64, IRType::String, IRType::F64, IRType::I64, IRType::I64]
                    } else {
                        vec![IRType::I64, IRType::I64, IRType::String, IRType::F64, IRType::I64]
                    };
                    reg.add("aal_rt_ai_call", params.clone(),
                            IRType::AiResponse(Box::new(IRType::String)));
                    ("aal_rt_ai_call".to_string(), params,
                     IRType::AiResponse(Box::new(IRType::String)))
                },
                Intrinsic::AiCallMany => {
                    reg.add("aal_rt_ai_call_many", vec![], IRType::AiManyResponse(Box::new(IRType::String)));
                    ("aal_rt_ai_call_many".to_string(), vec![], IRType::AiManyResponse(Box::new(IRType::String)))
                },
                Intrinsic::AiCallRace => {
                    reg.add("aal_rt_ai_call_race", vec![], IRType::AiRaceResponse(Box::new(IRType::String)));
                    ("aal_rt_ai_call_race".to_string(), vec![], IRType::AiRaceResponse(Box::new(IRType::String)))
                },
                Intrinsic::ContextNew => {
                    reg.add("aal_rt_ctx_new", vec![IRType::String, IRType::I64], IRType::I64);
                    ("aal_rt_ctx_new".to_string(), vec![IRType::String, IRType::I64], IRType::I64)
                },
                Intrinsic::ContextCurrent => {
                    reg.add("aal_rt_ctx_current", vec![], IRType::I64);
                    ("aal_rt_ctx_current".to_string(), vec![], IRType::I64)
                },
                Intrinsic::ContextBudget => {
                    reg.add("aal_rt_ctx_budget", vec![IRType::I64], IRType::I64);
                    ("aal_rt_ctx_budget".to_string(), vec![IRType::I64], IRType::I64)
                },
                Intrinsic::ExtractAiText => {
                    reg.add("aal_rt_extract_ai_text", vec![IRType::I64], IRType::String);
                    ("aal_rt_extract_ai_text".to_string(), vec![IRType::I64], IRType::String)
                },
                Intrinsic::ExtractAiVariant => {
                    reg.add("aal_rt_extract_ai_variant", vec![IRType::I64], IRType::I32);
                    ("aal_rt_extract_ai_variant".to_string(), vec![IRType::I64], IRType::I32)
                },
                Intrinsic::ExtractAiUsage => {
                    reg.add("aal_rt_extract_ai_usage", vec![IRType::I64], IRType::I64);
                    ("aal_rt_extract_ai_usage".to_string(), vec![IRType::I64], IRType::I64)
                },
                Intrinsic::ExtractAiReasoning => {
                    reg.add("aal_rt_extract_ai_reasoning", vec![IRType::I64], IRType::String);
                    ("aal_rt_extract_ai_reasoning".to_string(), vec![IRType::I64], IRType::String)
                },
                Intrinsic::ToolDispatch => {
                    reg.add("aal_rt_tool_dispatch", vec![IRType::String, IRType::String], IRType::String);
                    ("aal_rt_tool_dispatch".to_string(), vec![IRType::String, IRType::String], IRType::String)
                },
                Intrinsic::CapCheck => {
                    reg.add("aal_rt_cap_check", vec![IRType::String], IRType::Bool);
                    ("aal_rt_cap_check".to_string(), vec![IRType::String], IRType::Bool)
                },
                Intrinsic::ActorSpawn => {
                    reg.add("aal_rt_actor_spawn", vec![IRType::I64, IRType::I64], IRType::I64);
                    ("aal_rt_actor_spawn".to_string(), vec![IRType::I64, IRType::I64], IRType::I64)
                },
                Intrinsic::ActorSend => {
                    reg.add("aal_rt_actor_send", vec![IRType::I64, IRType::I64], IRType::Void);
                    ("aal_rt_actor_send".to_string(), vec![IRType::I64, IRType::I64], IRType::Void)
                },
                Intrinsic::ActorReceive => {
                    reg.add("aal_rt_actor_receive", vec![], IRType::I64);
                    ("aal_rt_actor_receive".to_string(), vec![], IRType::I64)
                },
                Intrinsic::Println => {
                    reg.add("aal_rt_println", vec![IRType::String], IRType::Void);
                    ("aal_rt_println".to_string(), vec![IRType::String], IRType::Void)
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
