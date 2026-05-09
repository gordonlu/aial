// llvm_backend.rs — LLVM IR text generator (zero dependencies)
// Produces .ll file, compiled by clang:  clang output.ll -laial_rt -o binary

use crate::ir::*;
use crate::ir_lower::RuntimeRegistry;
use std::collections::HashMap;

pub fn llvm_compile(module: &IRModule, reg: &RuntimeRegistry, output: &str) -> Result<(), String> {
    let mut out = String::new();
    out.push_str("; AIAL-generated LLVM IR\n");
    out.push_str("target triple = \"x86_64-unknown-linux-gnu\"\n\n");

    for rf in &reg.functions {
        let ret = llvm_type(&rf.ret);
        let params: Vec<String> = rf.params.iter().map(|p| llvm_type(p)).collect();
        out.push_str(&format!("declare {} @{}({})\n", ret, rf.name, params.join(", ")));
    }
    out.push_str("\n");

    for func in &module.functions {
        let ret = if func.name == "main" { "i32".to_string() } else { llvm_type(&func.return_type) };
        let p: Vec<String> = func.params.iter().map(|(_, t)| llvm_type(t)).collect();
        out.push_str(&format!("define {} @{}({}) {{\n", ret, func.name, p.join(", ")));

        // Collect ALL Values from instructions + terminators
        let mut all_vals: Vec<Value> = func.value_types.iter().map(|(v, _)| *v).collect();
        for b in &func.blocks {
            if let Some(ref term) = b.terminator {
                match term { Terminator::CondBr(v, _, _) | Terminator::Switch(v, _, _) => all_vals.push(*v), Terminator::Ret(Some(v)) => all_vals.push(*v), _ => {} }
            }
        }
        all_vals.sort_by_key(|v| v.0); all_vals.dedup_by_key(|v| v.0);

        // Register all Values with sequential LLVM names
        let mut var_map: HashMap<Value, String> = HashMap::new();
        for (i, v) in all_vals.iter().enumerate() { var_map.insert(*v, format!("%v{}", i)); }
        for (i, (v, _)) in func.params.iter().enumerate() { var_map.insert(*v, format!("%arg{}", i)); }

        for b in &func.blocks {
            if b.instrs.is_empty() && b.terminator.is_none() { continue; }
            out.push_str(&format!("\nb{}:\n", b.id.0));

            for (instr, opt_val) in &b.instrs {
                let vname = opt_val.and_then(|v| var_map.get(&v)).cloned().unwrap_or_else(|| format!("%tmp{}", b.id.0));
                match instr {
                    Instr::ConstInt(n) => out.push_str(&format!("  {} = add i64 0, {}\n", vname, n)),
                    Instr::ConstFloat(f) => { out.push_str(&format!("  {} = add i64 0, {}\n", vname, f64::to_bits(*f))); }
                    Instr::ConstBool(b) => out.push_str(&format!("  {} = add i64 0, {}\n", vname, if *b { 1 } else { 0 })),
                    Instr::ConstNull => out.push_str(&format!("  {} = add i64 0, 0\n", vname)),
                    Instr::ConstString(s) => { let idx = module.strings.iter().position(|x| x == s).unwrap_or(0); out.push_str(&format!("  {} = add i64 0, {}\n", vname, idx)); }
                    Instr::Alloca(_) => out.push_str(&format!("  {} = alloca i64\n", vname)),
                    Instr::Load(ptr) => { let p = lookup(&var_map, ptr); out.push_str(&format!("  {} = load i64, i64* {}\n", vname, p)); }
                    Instr::Store(ptr, val_v) => { let p = lookup(&var_map, ptr); let v = lookup(&var_map, val_v); out.push_str(&format!("  store i64 {}, i64* {}\n  {} = add i64 0, 0\n", v, p, vname)); }
                    Instr::BinOp(op, l, r) => { let lv = lookup(&var_map, l); let rv = lookup(&var_map, r); let o = binop_str(op); out.push_str(&format!("  {} = {} i64 {}, {}\n", vname, o, lv, rv)); }
                    Instr::Cmp(op, l, r) => { let lv = lookup(&var_map, l); let rv = lookup(&var_map, r); let c = cmp_str(op); out.push_str(&format!("  {} = icmp {} i64 {}, {}\n", vname, c, lv, rv)); }
                    Instr::ExternCall { name, args, .. } => { let a: Vec<String> = args.iter().map(|a| lookup(&var_map, a)).collect(); out.push_str(&format!("  {} = call i64 @{}({})\n", vname, name, a.join(", "))); }
                    _ => out.push_str(&format!("  {} = add i64 0, 0\n", vname)),
                }
            }

            match &b.terminator {
                Some(Terminator::Br(target)) => out.push_str(&format!("  br label %b{}\n", target.0)),
                Some(Terminator::CondBr(cond, t, f)) => { let cv = lookup(&var_map, cond); out.push_str(&format!("  br i1 {}, label %b{}, label %b{}\n", cv, t.0, f.0)); }
                Some(Terminator::Ret(val)) => {
                    if let Some(v) = val {
                        let mut rv = lookup(&var_map, v);
                        let ret_ty = if func.name == "main" { "i32".to_string() } else { llvm_type(&func.return_type) };
                        // Trunc i64 → i32 for main's C ABI
                        if func.name == "main" {
                            let tmp = format!("%trunc{}", v.0);
                            out.push_str(&format!("  {} = trunc i64 {} to i32\n", tmp, rv));
                            rv = tmp;
                        }
                        out.push_str(&format!("  ret {} {}\n", ret_ty, rv));
                    } else {
                        out.push_str(if func.name == "main" { "  ret i32 0\n" } else { "  ret void\n" });
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

fn lookup(map: &HashMap<Value, String>, v: &Value) -> String { map.get(v).cloned().unwrap_or("0".into()) }
fn binop_str(op: &crate::ast::BinOp) -> &str { match op { crate::ast::BinOp::Add => "add", crate::ast::BinOp::Sub => "sub", crate::ast::BinOp::Mul => "mul", crate::ast::BinOp::Div => "sdiv", crate::ast::BinOp::Rem => "srem", _ => "add", } }
fn cmp_str(op: &crate::ast::BinOp) -> &str { use crate::ast::BinOp; match op { BinOp::Eq => "eq", BinOp::Ne => "ne", BinOp::Lt => "slt", BinOp::Gt => "sgt", BinOp::Le => "sle", BinOp::Ge => "sge", _ => "eq", } }
fn llvm_type(ty: &IRType) -> String { match ty { IRType::I64|IRType::String|IRType::Bool|IRType::I32|IRType::Void => "i64".into(), IRType::F64|IRType::F32 => "double".into(), _ => "i64".into() } }
