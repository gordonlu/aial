// aot_backend.rs — Cranelift AOT (ahead-of-time) compilation backend
// Generates a .o file from lowered AIAL-IR. Link with aial_rt.o to produce an executable.

use crate::ir::*;
use crate::ir_lower::RuntimeRegistry;
use cranelift_codegen::ir::{types, AbiParam, InstBuilder, Signature};
use cranelift_codegen::ir::entities::UserExternalNameRef;
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::settings::{self, Configurable};
use cranelift_codegen::Context;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule, ObjectProduct};
use std::collections::HashMap;

pub fn aot_compile(module: &IRModule, reg: &RuntimeRegistry, output: &str) -> Result<(), String> {
    let mut flag_builder = settings::builder();
    flag_builder.set("use_colocated_libcalls", "true").unwrap();
    flag_builder.set("is_pic", "true").unwrap(); // PIC for linking
    let flags = settings::Flags::new(flag_builder);

    let isa_builder = cranelift_native::builder()
        .map_err(|e| format!("native ISA: {}", e))?;
    let isa = isa_builder.finish(flags)
        .map_err(|e| format!("ISA: {}", e))?;

    let obj_builder = ObjectBuilder::new(
        isa, "aial_output", cranelift_module::default_libcall_names(),
    ).map_err(|e| format!("ObjectBuilder: {}", e))?;
    let mut obj_module = ObjectModule::new(obj_builder);

    // Declare module functions (the user's code)
    let mut func_ids: HashMap<String, cranelift_module::FuncId> = HashMap::new();
    for func in &module.functions {
        let mut sig = Signature::new(CallConv::SystemV);
        for (_, t) in &func.params {
            sig.params.push(AbiParam::new(ir_type_to_cl(t)));
        }
        if func.return_type != IRType::Void {
            sig.returns.push(AbiParam::new(ir_type_to_cl(&func.return_type)));
        }
        let id = obj_module
            .declare_function(&func.name, Linkage::Export, &sig)
            .map_err(|e| format!("declare export `{}`: {}", func.name, e))?;
        func_ids.insert(func.name.clone(), id);
    }

    // Compile each function (shared logic with JIT)
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

        let mut fb = FunctionBuilder::new(&mut ctx.func, &mut builder_ctx);

        // Build blocks
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

        // Declare Cranelift Variables for all IR Values
        let mut var_map: HashMap<Value, Variable> = HashMap::new();
        for (v, ir_ty) in &func.value_types {
            let var = fb.declare_var(ir_type_to_cl(ir_ty));
            var_map.insert(*v, var);
        }
        for (i, (v, _)) in func.params.iter().enumerate() {
            fb.def_var(var_map[v], fb.block_params(entry_block)[i]);
        }

        // Pre-import extern call references
        let mut ref_cache: HashMap<String, cranelift_codegen::ir::FuncRef> = HashMap::new();
        for b in &func.blocks {
            for (instr, _) in &b.instrs {
                if let Instr::ExternCall { name, .. } = instr {
                    if !ref_cache.contains_key(name) {
                        if let Some(rt_fn) = reg.functions.iter().find(|f| &f.name == name) {
                            let mut sig = Signature::new(CallConv::SystemV);
                            for p in &rt_fn.params { sig.params.push(AbiParam::new(ir_type_to_cl(p))); }
                            if rt_fn.ret != IRType::Void { sig.returns.push(AbiParam::new(ir_type_to_cl(&rt_fn.ret))); }
                            let sig_ref = fb.import_signature(sig);
                            let ext = cranelift_codegen::ir::ExtFuncData {
                                name: cranelift_codegen::ir::ExternalName::user(UserExternalNameRef::from_u32(0)),
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
            if cl_block != entry_block { fb.seal_block(cl_block); }

            for (instr, opt_val) in &b.instrs {
                let cl_val = translate_instr(&mut fb, instr, &var_map, &ref_cache)?;
                if let Some(ir_val) = opt_val {
                    if let Some(&var) = var_map.get(ir_val) { fb.def_var(var, cl_val); }
                }
            }

            match &b.terminator {
                Some(Terminator::Br(target)) => { fb.ins().jump(block_map[target], &[]); }
                Some(Terminator::CondBr(cond, t, f)) => {
                    let cond_v = fb.use_var(var_map[cond]);
                    fb.ins().brif(cond_v, block_map[t], &[], block_map[f], &[]);
                }
                Some(Terminator::Ret(val)) => {
                    if let Some(v) = val {
                        let ret_v = fb.use_var(var_map[v]);
                        fb.ins().return_(&[ret_v]);
                    } else { fb.ins().return_(&[]); }
                }
                Some(Terminator::Unreachable) | Some(Terminator::Switch(..)) => {
                    fb.ins().return_(&[]);
                }
                None => {}
            }
        }

        fb.finalize();
        obj_module.define_function(func_id, &mut ctx)
            .map_err(|e| format!("define `{}`: {}", func.name, e))?;
    }

    let obj: ObjectProduct = obj_module.finish();
    let obj_bytes = obj.emit().map_err(|e| format!("emit object: {}", e))?;
    std::fs::write(output, obj_bytes).map_err(|e| format!("write {}: {}", output, e))?;
    Ok(())
}

/// Shared instruction translation (identical to JIT backend).
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
        Instr::Load(ptr) => vars.get(ptr).map(|v| fb.use_var(*v)).unwrap_or_else(|| fb.ins().iconst(types::I64, 0)),
        Instr::ExternCall { name, args, .. } => {
            if let Some(fr) = refs.get(name) {
                let cl_args: Vec<cranelift_codegen::ir::Value> =
                    args.iter().map(|a| fb.use_var(vars[a])).collect();
                let call = fb.ins().call(*fr, &cl_args);
                let results = fb.inst_results(call);
                if let Some(&r) = results.first() { r } else { fb.ins().iconst(types::I64, 0) }
            } else { fb.ins().iconst(types::I64, 0) }
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
