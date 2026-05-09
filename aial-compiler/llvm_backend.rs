// llvm_backend.rs — LLVM IR text generator (zero dependencies)
// Produces .ll file, compiled by clang:  clang output.ll -laial_rt -o binary

use crate::ir::*;
use crate::ir_lower::RuntimeRegistry;
use std::collections::HashMap;

pub fn llvm_compile(module: &IRModule, reg: &RuntimeRegistry, output: &str) -> Result<(), String> {
    let mut out = String::new();

    // Module header
    out.push_str("; AIAL-generated LLVM IR\n");
    out.push_str("target triple = \"x86_64-unknown-linux-gnu\"\n\n");

    // Declare runtime externs (C ABI)
    for rf in &reg.functions {
        let ret = llvm_type(&rf.ret);
        let params: Vec<String> = rf.params.iter().map(|p| llvm_type(p)).collect();
        out.push_str(&format!("declare {} @{}({})\n", ret, rf.name, params.join(", ")));
    }
    out.push_str("\n");

    // Compile each function
    for func in &module.functions {
        let ret = if func.return_type == IRType::Void { "void".to_string() } else { llvm_type(&func.return_type) };
        let params: Vec<String> = func.params.iter().map(|(_, t)| llvm_type(t)).collect();
        out.push_str(&format!("define {} @{}({}) {{\n", ret, func.name, params.join(", ")));

        let mut var_map: HashMap<Value, String> = HashMap::new();
        let mut var_counter: usize = 0;
        let mut next_var = move || { let v = var_counter; var_counter += 1; format!("%v{}", v) };

        for (i, (v, _)) in func.params.iter().enumerate() {
            var_map.insert(*v, format!("%arg{}", i));
        }

        for b in &func.blocks {
            if b.instrs.is_empty() && b.terminator.is_none() { continue; } // skip dead blocks
            out.push_str(&format!("\nb{}:\n", b.id.0));

            for (instr, opt_val) in &b.instrs {
                let vname = next_var();
                match instr {
                    Instr::ConstInt(n) => {
                        out.push_str(&format!("  {} = add i64 0, {}\n", vname, n));
                        if let Some(v) = opt_val { var_map.insert(*v, vname.clone()); }
                    }
                    Instr::ConstFloat(f) => {
                        let bits = f64::to_bits(*f);
                        out.push_str(&format!("  {} = add i64 0, {}\n", vname, bits));
                        if let Some(v) = opt_val { var_map.insert(*v, vname.clone()); }
                    }
                    Instr::ConstString(s) => {
                        let idx = module.strings.iter().position(|x| x == s).unwrap_or(0);
                        out.push_str(&format!("  {} = add i64 0, {}\n", vname, idx));
                        if let Some(v) = opt_val { var_map.insert(*v, vname.clone()); }
                    }
                    Instr::Alloca(_) => {
                        out.push_str(&format!("  {} = alloca i64\n", vname));
                        if let Some(v) = opt_val { var_map.insert(*v, vname.clone()); }
                    }
                    Instr::Load(ptr) => {
                        let p = var_map.get(ptr).cloned().unwrap_or_else(|| "null".to_string());
                        out.push_str(&format!("  {} = load i64, i64* {}\n", vname, p));
                        if let Some(v) = opt_val { var_map.insert(*v, vname.clone()); }
                    }
                    Instr::Store(ptr, val_v) => {
                        let p = var_map.get(ptr).cloned().unwrap_or("null".to_string());
                        let v = var_map.get(val_v).cloned().unwrap_or("0".to_string());
                        // v is already a %vN name, just need the value
                        out.push_str(&format!("  store i64 {}, i64* {}\n", v, p));
                        out.push_str(&format!("  {} = add i64 0, 0\n", vname));
                        if let Some(v) = opt_val { var_map.insert(*v, vname.clone()); }
                    }
                    Instr::BinOp(op, l, r) => {
                        let lv = var_map.get(l).cloned().unwrap_or("i64 0".to_string());
                        let rv = var_map.get(r).cloned().unwrap_or("i64 0".to_string());
                        let llvm_op = match op { crate::ast::BinOp::Add => "add", crate::ast::BinOp::Sub => "sub", crate::ast::BinOp::Mul => "mul", crate::ast::BinOp::Div => "sdiv", crate::ast::BinOp::Rem => "srem", _ => "add", };
                        out.push_str(&format!("  {} = {} i64 {}, {}\n", vname, llvm_op, lv, rv));
                    }
                    Instr::Cmp(op, l, r) => {
                        let lv = var_map.get(l).cloned().unwrap_or("i64 0".to_string());
                        let rv = var_map.get(r).cloned().unwrap_or("i64 0".to_string());
                        use crate::ast::BinOp;
                        let cond = match op { BinOp::Eq => "eq", BinOp::Ne => "ne", BinOp::Lt => "slt", BinOp::Gt => "sgt", BinOp::Le => "sle", BinOp::Ge => "sge", _ => "eq" };
                        out.push_str(&format!("  {} = icmp {} i64 {}, {}\n", vname, cond, lv, rv));
                    }
                    Instr::ExternCall { name, args, .. } => {
                        let a: Vec<String> = args.iter().map(|a| {
                            var_map.get(a).cloned().unwrap_or("0".to_string())
                        }).collect();
                        let ret_type = "i64";
                        out.push_str(&format!("  {} = call {} @{}({})\n", vname, ret_type, name, a.join(", ")));
                        if let Some(v) = opt_val { var_map.insert(*v, vname.clone()); }
                    }
                    _ => {
                        out.push_str(&format!("  {} = add i64 0, 0\n", vname));
                        if let Some(v) = opt_val { var_map.insert(*v, vname.clone()); }
                    }
                }
            }

            // Terminator
            match &b.terminator {
                Some(Terminator::Br(target)) => out.push_str(&format!("  br label %b{}\n", target.0)),
                Some(Terminator::CondBr(cond, t, f)) => {
                    let cv = var_map.get(cond).cloned().unwrap_or("i64 0".to_string());
                    out.push_str(&format!("  br i1 {}, label %b{}, label %b{}\n", cv, t.0, f.0));
                }
                Some(Terminator::Ret(val)) => {
                    if let Some(v) = val {
                        let rv = var_map.get(v).cloned().unwrap_or("i64 0".to_string());
                        out.push_str(&format!("  ret {} {}\n", llvm_type(&func.return_type), rv));
                    } else {
                        out.push_str("  ret void\n");
                    }
                }
                Some(Terminator::Unreachable) => out.push_str("  unreachable\n"),
                _ => {}
            }
        }

        out.push_str("}\n\n");
    }

    std::fs::write(output, &out).map_err(|e| format!("write {}: {}", output, e))?;
    Ok(())
}

fn llvm_type(ty: &IRType) -> String {
    match ty { IRType::I64 | IRType::String | IRType::Bool | IRType::I32 | IRType::Void => "i64".into(), IRType::F64 | IRType::F32 => "double".into(), _ => "i64".into() }
}
