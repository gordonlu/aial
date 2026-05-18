// jit_backend.rs — Cranelift JIT backend for AIAL
// Translates lowered AAL-IR to machine code via cranelift-jit.

use crate::ir::*;
use crate::ir_lower::RuntimeRegistry;
use cranelift_codegen::ir::{types, AbiParam, ExtFuncData, InstBuilder, Signature};
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::settings::{self, Configurable};
use cranelift_codegen::Context;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use std::collections::HashMap;

// ──── Runtime callback stubs (extern "C" for JIT symbol resolution) ────
// Functions are provided by aial-rt crate — references only here.

extern "C" {
    fn aial_rt_ai_call(_m: i64, _c: i64, _p: i64, _t: f64, _tk: i64, _fmt: i64) -> i64;
    fn aial_rt_println(_m: i64);
    fn aial_rt_extract_ai_text(_r: i64) -> i64;
    fn aial_rt_ctx_new(_p: i64, _b: i64, _s: i64, _ws: i64) -> i64;
    fn aial_rt_ctx_budget(_c: i64) -> i64;
}

// ──── Main JIT compilation entry point ────

pub fn jit_run(module: &IRModule, reg: &RuntimeRegistry) -> Result<(), String> {
    let mut flag_builder = settings::builder();
    flag_builder.set("use_colocated_libcalls", "true").unwrap();
    flag_builder.set("is_pic", "false").unwrap();
    let flags = settings::Flags::new(flag_builder);

    let isa_builder =
        cranelift_native::builder().map_err(|e| format!("native ISA: {}", e))?;
    let isa = isa_builder
        .finish(flags)
        .map_err(|e| format!("ISA: {}", e))?;

    let mut jit_builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
    jit_builder.symbols(vec![
        ("aial_rt_ai_call",          aial_rt_ai_call as *const u8),
        ("aial_rt_println",          aial_rt_println as *const u8),
        ("aial_rt_extract_ai_text",  aial_rt_extract_ai_text as *const u8),
        ("aial_rt_ctx_new",          aial_rt_ctx_new as *const u8),
        ("aial_rt_ctx_budget",       aial_rt_ctx_budget as *const u8),
    ]);
    // Register runtime symbols via callback lookup (before JITModule consumes builder)
    let runtime_ptrs: std::sync::Arc<HashMap<String, usize>> = std::sync::Arc::new({
        let mut m = HashMap::new();
        m.insert("aial_rt_ai_call".to_string(), aial_rt_ai_call as *const () as usize);
        m.insert("aial_rt_println".to_string(), aial_rt_println as *const () as usize);
        m.insert("aial_rt_extract_ai_text".to_string(), aial_rt_extract_ai_text as *const () as usize);
        m.insert("aial_rt_ctx_new".to_string(), aial_rt_ctx_new as *const () as usize);
        m.insert("aial_rt_ctx_budget".to_string(), aial_rt_ctx_budget as *const () as usize);
        m
    });
    let ptrs = runtime_ptrs.clone();
    jit_builder.symbol_lookup_fn(Box::new(move |name: &str| -> Option<*const u8> {
        ptrs.get(name).map(|&p| p as *const u8)
    }));
    let mut jit = JITModule::new(jit_builder);

    // Track runtime function indices for user-named ExternalName mapping
    let mut runtime_name_indices: HashMap<String, u32> = HashMap::new();
    for rt_fn in &reg.functions {
        let idx = runtime_name_indices.len() as u32;
        runtime_name_indices.insert(rt_fn.name.clone(), idx);
    }

    // Declare module functions
    let mut func_ids: HashMap<String, cranelift_module::FuncId> = HashMap::new();
    for func in &module.functions {
        let mut sig = Signature::new(CallConv::SystemV);
        for (_, t) in &func.params {
            sig.params.push(AbiParam::new(ir_type_to_cl(t)));
        }
        if func.return_type != IRType::Void {
            sig.returns.push(AbiParam::new(ir_type_to_cl(&func.return_type)));
        }
        let id = jit
            .declare_function(&func.name, Linkage::Export, &sig)
            .map_err(|e| format!("declare `{}`: {}", func.name, e))?;
        func_ids.insert(func.name.clone(), id);
    }

    // Compile each function
    for func in &module.functions {
        let func_id = func_ids[&func.name];
        let mut ctx = Context::new();
        let mut builder_ctx = FunctionBuilderContext::new();

        let mut sig = Signature::new(CallConv::SystemV);
        for (_, t) in &func.params {
            sig.params.push(AbiParam::new(ir_type_to_cl(t)));
        }
        if func.return_type != IRType::Void {
            sig.returns.push(AbiParam::new(ir_type_to_cl(&func.return_type)));
        }
        ctx.func.signature = sig;

        // Pre-register all runtime function user names, save the actual UserExternalNameRefs
        let mut user_refs: HashMap<String, cranelift_codegen::ir::UserExternalNameRef> = HashMap::new();
        for b in &func.blocks {
            for (instr, _) in &b.instrs {
                if let Instr::ExternCall { name, .. } = instr {
                    if runtime_name_indices.contains_key(name.as_str()) && !user_refs.contains_key(name.as_str()) {
                        let idx = runtime_name_indices[name.as_str()];
                        let user_name = cranelift_codegen::ir::UserExternalName::new(0, idx);
                        let reff = ctx.func.params.ensure_user_func_name(user_name);
                        user_refs.insert(name.clone(), reff);
                    }
                }
            }
        }

        let mut fb = FunctionBuilder::new(&mut ctx.func, &mut builder_ctx);

        // Build block map
        let entry_block = fb.create_block();
        let mut block_map: HashMap<BlockId, cranelift_codegen::ir::Block> = HashMap::new();
        block_map.insert(func.entry, entry_block);
        for b in &func.blocks {
            if !block_map.contains_key(&b.id) {
                block_map.insert(b.id, fb.create_block());
            }
        }

        fb.append_block_params_for_function_params(entry_block);
        fb.switch_to_block(entry_block);
        fb.seal_block(entry_block);

        // Declare a Cranelift Variable for every IR Value
        let mut var_map: HashMap<Value, Variable> = HashMap::new();
        for (v, ir_ty) in &func.value_types {
            let var = fb.declare_var(ir_type_to_cl(ir_ty));
            var_map.insert(*v, var);
        }
        // Initialize function parameters
        for (i, (v, _)) in func.params.iter().enumerate() {
            fb.def_var(var_map[v], fb.block_params(entry_block)[i]);
        }

        // Register runtime extern functions via user_named_funcs + symbol_lookup
        let mut ref_cache: HashMap<String, cranelift_codegen::ir::FuncRef> = HashMap::new();
        for b in &func.blocks {
            for (instr, _) in &b.instrs {
                if let Instr::ExternCall { name, .. } = instr {
                    if !ref_cache.contains_key(name) && runtime_name_indices.contains_key(name.as_str()) {
                        if let Some(rt_fn) = reg.functions.iter().find(|f| &f.name == name) {
                            let mut sig = Signature::new(CallConv::SystemV);
                            for p in &rt_fn.params { sig.params.push(AbiParam::new(ir_type_to_cl(p))); }
                            if rt_fn.ret != IRType::Void { sig.returns.push(AbiParam::new(ir_type_to_cl(&rt_fn.ret))); }
                            let sig_ref = fb.import_signature(sig);
                            let reff = user_refs[name.as_str()];
                            let ext = ExtFuncData {
                                name: cranelift_codegen::ir::ExternalName::User(reff),
                                signature: sig_ref, colocated: false, patchable: false,
                            };
                            ref_cache.insert(name.clone(), fb.import_function(ext));
                        }
                    }
                }
            }
        }

        // Translate blocks
        for b in &func.blocks {
            let cl_block = block_map[&b.id];
            fb.switch_to_block(cl_block);
            if cl_block != entry_block {
                fb.seal_block(cl_block);
            }

            for (instr, opt_val) in &b.instrs {
                let cl_val = translate_instr(&mut fb, instr, &var_map, &ref_cache)?;
                // Store result to the declared variable
                if let Some(ir_val) = opt_val {
                    if let Some(&var) = var_map.get(ir_val) {
                        fb.def_var(var, cl_val);
                    }
                }
            }

            match &b.terminator {
                Some(Terminator::Br(target)) => {
                    fb.ins().jump(block_map[target], &[]);
                }
                Some(Terminator::CondBr(cond, t, f)) => {
                    let cond_v = fb.use_var(var_map[cond]);
                    fb.ins().brif(cond_v, block_map[t], &[], block_map[f], &[]);
                }
                Some(Terminator::Ret(val)) => {
                    if let Some(v) = val {
                        let ret_v = fb.use_var(var_map[v]);
                        fb.ins().return_(&[ret_v]);
                    } else {
                        fb.ins().return_(&[]);
                    }
                }
                Some(Terminator::Unreachable) | Some(Terminator::Switch(..)) => {
                    fb.ins().return_(&[]);
                }
                None => {}
            }
        }

        fb.finalize();
        jit.define_function(func_id, &mut ctx)
            .map_err(|e| format!("define `{}`: {}", func.name, e))?;
    }

    jit.finalize_definitions()
        .map_err(|e| format!("finalize: {}", e))?;
    if let Some(&main_id) = func_ids.get("main") {
        let main_ptr: *const u8 = jit.get_finalized_function(main_id);
        // Select calling convention based on return type
        let main_fn = module.functions.iter().find(|f| f.name == "main").unwrap();
        if main_fn.return_type == IRType::Void {
            let main_fn: extern "C" fn() = unsafe { std::mem::transmute(main_ptr) };
            main_fn();
        } else {
            let main_fn: extern "C" fn() -> i64 = unsafe { std::mem::transmute(main_ptr) };
            main_fn();
        }
    }
    Ok(())
}

fn translate_instr(
    fb: &mut FunctionBuilder,
    instr: &Instr,
    vars: &HashMap<Value, Variable>,
    refs: &HashMap<String, cranelift_codegen::ir::FuncRef>,
) -> Result<cranelift_codegen::ir::Value, String> {
    let val = match instr {
        Instr::ConstInt(n) => fb.ins().iconst(types::I64, *n),
        Instr::ConstFloat(f) => fb.ins().f64const(*f),
        Instr::ConstBool(b) => fb.ins().iconst(types::I8, if *b { 1 } else { 0 }),
        Instr::ConstNull => fb.ins().iconst(types::I64, 0),
        Instr::ConstString(_) => fb.ins().iconst(types::I64, 0),
        Instr::Alloca(_) => fb.ins().iconst(types::I64, 0),
        Instr::Store(ptr, val_v) => {
            let val = vars.get(val_v).map(|v| fb.use_var(*v)).unwrap_or_else(|| fb.ins().iconst(types::I64, 0));
            if let Some(&v) = vars.get(ptr) { fb.def_var(v, val); }
            fb.ins().iconst(types::I64, 0)
        }
        Instr::Load(ptr) => {
            if let Some(var) = vars.get(ptr) {
                fb.use_var(*var)
            } else {
                fb.ins().iconst(types::I64, 0)
            }
        }
        Instr::ExternCall { name, args, .. } => {
            if let Some(fr) = refs.get(name) {
                let cl_args: Vec<cranelift_codegen::ir::Value> =
                    args.iter().map(|a| fb.use_var(vars[a])).collect();
                let call = fb.ins().call(*fr, &cl_args);
                let results = fb.inst_results(call);
                if let Some(&r) = results.first() { r } else { fb.ins().iconst(types::I64, 0) }
            } else {
                fb.ins().iconst(types::I64, 0)
            }
        }
        _ => fb.ins().iconst(types::I64, 0),
    };
    Ok(val)
}

fn ir_type_to_cl(ty: &IRType) -> cranelift_codegen::ir::Type {
    match ty {
        IRType::I32 => types::I32,
        IRType::I64 => types::I64,
        IRType::F32 => types::F32,
        IRType::F64 => types::F64,
        IRType::Bool => types::I8,
        _ => types::I64,
    }
}
