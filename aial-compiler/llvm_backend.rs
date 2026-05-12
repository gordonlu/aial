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
        let ret = if rf.ret == IRType::Bool { "i64".to_string() } else { llvm_type(&rf.ret) };
        let params: Vec<String> = rf.params.iter().map(|p| llvm_type(p)).collect();
        out.push_str(&format!("declare {} @{}({})\n", ret, rf.name, params.join(", ")));
    }
    if !module.strings.is_empty() {
        out.push_str("declare void @aial_rt_string_register(i64, i8*)\n");
    }
    out.push_str("\n");

    // Generate global string constants for compile-time string table
    if !module.strings.is_empty() {
        for (i, s) in module.strings.iter().enumerate() {
            let escaped = s.replace('\\', "\\\\").replace('"', "\\22").replace('\n', "\\0A").replace('\r', "\\0D").replace('\t', "\\09");
            let len = s.len() + 1; // +1 for null terminator
            out.push_str(&format!("@.str{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1\n", i, len, escaped));
        }
        out.push_str("\n");
    }

    for func in &module.functions {
        let ret = if func.name == "main" { "i32".to_string() } else { llvm_type(&func.return_type) };
        let p: Vec<String> = func.params.iter().enumerate().map(|(i, (_, t))| format!("{} %arg{}", llvm_type(t), i)).collect();
        out.push_str(&format!("define {} @{}({}) {{\n", ret, func.name, p.join(", ")));

        // Collect ALL Values from instructions + terminators
        let mut all_vals: Vec<Value> = func.value_types.iter().map(|(v, _)| *v).collect();
        for b in &func.blocks {
            if let Some(ref term) = b.terminator {
                match term { Terminator::CondBr(v, _, _) | Terminator::Switch(v, _, _) => all_vals.push(*v), Terminator::Ret(Some(v)) => all_vals.push(*v), _ => {} }
            }
        }
        all_vals.sort_by_key(|v| v.0); all_vals.dedup_by_key(|v| v.0);

        // Register all Values with sequential LLVM names, track LLVM types
        let mut var_map: HashMap<Value, String> = HashMap::new();
        let mut type_map: HashMap<Value, String> = HashMap::new();
        for (i, v) in all_vals.iter().enumerate() {
            let name = format!("%v{}", i);
            var_map.insert(*v, name.clone());
            // Infer LLVM type from the IR type stored in value_types
            if let Some((_, ir_ty)) = func.value_types.iter().find(|(vv, _)| vv == v) {
                type_map.insert(*v, llvm_type(ir_ty));
            }
        }
        for (i, (v, t)) in func.params.iter().enumerate() {
            var_map.insert(*v, format!("%arg{}_addr", i));
            type_map.insert(*v, llvm_type(t));
        }

        for b in &func.blocks {
            if b.instrs.is_empty() && b.terminator.is_none() {
                out.push_str(&format!("\nb{}:\n", b.id.0));
                if ret == "void" { out.push_str("  ret void\n"); }
                else { out.push_str(&format!("  ret {} 0\n", ret)); }
                continue;
            }
            out.push_str(&format!("\nb{}:\n", b.id.0));

            // Emit alloca+store for params in entry block so Load works
            if b.id == func.entry {
                // Init compile-time string table before anything else
                if func.name == "main" && !module.strings.is_empty() {
                    for (i, s) in module.strings.iter().enumerate() {
                        let len = s.len() + 1;
                        out.push_str(&format!("  %str_init_{} = getelementptr inbounds [{} x i8], [{} x i8]* @.str{}, i32 0, i32 0\n", i, len, len, i));
                        out.push_str(&format!("  call void @aial_rt_string_register(i64 {}, i8* %str_init_{})\n", i, i));
                    }
                }
                for (i, (v, _)) in func.params.iter().enumerate() {
                    let addr = format!("%arg{}_addr", i);
                    out.push_str(&format!("  {} = alloca i64\n", addr));
                    out.push_str(&format!("  store i64 %arg{}, i64* {}\n", i, addr));
                }
            }

            for (instr, opt_val) in &b.instrs {
                let vname = opt_val.and_then(|v| var_map.get(&v)).cloned().unwrap_or_else(|| format!("%tmp{}", b.id.0));
                let lty = instr_llvm_type(instr);
                if let Some(v) = opt_val { type_map.insert(*v, lty.clone()); }
                match instr {
                    Instr::ConstInt(n) => out.push_str(&format!("  {} = add i64 0, {}\n", vname, n)),
                    Instr::ConstFloat(f) => { out.push_str(&format!("  {} = add i64 0, {}\n", vname, f64::to_bits(*f))); }
                    Instr::ConstBool(b) => out.push_str(&format!("  {} = add i1 0, {}\n", vname, if *b { 1 } else { 0 })),
                    Instr::ConstNull => out.push_str(&format!("  {} = add i64 0, 0\n", vname)),
                    Instr::ConstString(s) => { let idx = module.strings.iter().position(|x| x == s).unwrap_or(0); out.push_str(&format!("  {} = add i64 0, {}\n", vname, idx)); }
                    Instr::Alloca(_) => out.push_str(&format!("  {} = alloca i64\n", vname)),
                    Instr::Load(ptr) => { let p = lookup(&var_map, ptr); out.push_str(&format!("  {} = load i64, i64* {}\n", vname, p)); }
                    Instr::Store(ptr, val_v) => { let p = lookup(&var_map, ptr); let v = lookup(&var_map, val_v); out.push_str(&format!("  store i64 {}, i64* {}\n  {} = add i64 0, 0\n", v, p, vname)); }
                    Instr::Cmp(op, l, r) => { let lv = lookup(&var_map, l); let rv = lookup(&var_map, r); let c = cmp_str(op); out.push_str(&format!("  {} = icmp {} i64 {}, {}\n", vname, c, lv, rv)); }
                    Instr::UnOp(op, val) => {
                        let vv = lookup(&var_map, val);
                        match op {
                            crate::ast::UnOp::Not => out.push_str(&format!("  {} = xor i1 {}, true\n", vname, vv)),
                            crate::ast::UnOp::Neg => out.push_str(&format!("  {} = sub i64 0, {}\n", vname, vv)),
                        }
                    }
                    Instr::BinOp(op, l, r) => {
                        let lv = lookup(&var_map, l); let rv = lookup(&var_map, r);
                        let o = binop_str(op);
                        let ty = match op { crate::ast::BinOp::And | crate::ast::BinOp::Or => "i1", _ => "i64" };
                        out.push_str(&format!("  {} = {} {} {}, {}\n", vname, o, ty, lv, rv));
                    }
                    Instr::ExternCall { name, args, ret_ty, .. } => {
                        let a: Vec<String> = args.iter().map(|a| format!("i64 {}", lookup(&var_map, a))).collect();
                        let rty = llvm_type(ret_ty);
                        if rty == "void" { out.push_str(&format!("  call void @{}({})\n  {} = add i64 0, 0\n", name, a.join(", "), vname)); }
                        else { out.push_str(&format!("  {} = call {} @{}({})\n", vname, rty, name, a.join(", "))); }
                    }
                    Instr::UserCall { name, args, .. } => { let a: Vec<String> = args.iter().map(|a| format!("i64 {}", lookup(&var_map, a))).collect(); out.push_str(&format!("  {} = call i64 @{}({})\n", vname, name, a.join(", "))); }
                    Instr::Call { args, .. } => { let a: Vec<String> = args.iter().map(|a| format!("i64 {}", lookup(&var_map, a))).collect(); out.push_str(&format!("  {} = call i64 @unknown({})\n", vname, a.join(", "))); }
                    _ => out.push_str(&format!("  {} = add i64 0, 0\n", vname)),
                }
            }

            match &b.terminator {
                Some(Terminator::Br(target)) => out.push_str(&format!("  br label %b{}\n", target.0)),
                Some(Terminator::CondBr(cond, t, f)) => {
                    let cv = lookup(&var_map, cond);
                    let cond_ty = type_map.get(cond).map(|s| s.as_str()).unwrap_or("i64");
                    let iv = if cond_ty == "i1" { cv } else {
                        let tmp = format!("%icond{}", cond.0);
                        out.push_str(&format!("  {} = icmp ne i64 {}, 0\n", tmp, cv));
                        tmp
                    };
                    out.push_str(&format!("  br i1 {}, label %b{}, label %b{}\n", iv, t.0, f.0));
                }
                Some(Terminator::Ret(val)) => {
                    if let Some(v) = val {
                        let mut rv = lookup(&var_map, v);
                        let ret_ty = if func.name == "main" { "i32".to_string() } else { llvm_type(&func.return_type) };
                        if func.name == "main" {
                            let tmp = format!("%trunc{}", v.0);
                            out.push_str(&format!("  {} = trunc i64 {} to i32\n", tmp, rv));
                            rv = tmp;
                        }
                        out.push_str(&format!("  ret {} {}\n", ret_ty, rv));
                    } else {
                        if func.name == "main" { out.push_str("  ret i32 0\n"); }
                        else if ret == "void" { out.push_str("  ret void\n"); }
                        else { out.push_str(&format!("  ret {} 0\n", ret)); }
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
fn binop_str(op: &crate::ast::BinOp) -> &str { match op { crate::ast::BinOp::Add => "add", crate::ast::BinOp::Sub => "sub", crate::ast::BinOp::Mul => "mul", crate::ast::BinOp::Div => "sdiv", crate::ast::BinOp::Rem => "srem", crate::ast::BinOp::And => "and", crate::ast::BinOp::Or => "or", _ => "add", } }
fn cmp_str(op: &crate::ast::BinOp) -> &str { use crate::ast::BinOp; match op { BinOp::Eq => "eq", BinOp::Ne => "ne", BinOp::Lt => "slt", BinOp::Gt => "sgt", BinOp::Le => "sle", BinOp::Ge => "sge", _ => "eq", } }
fn llvm_type(ty: &IRType) -> String { match ty { IRType::Bool => "i1".into(), IRType::F64|IRType::F32 => "double".into(), IRType::Void => "void".into(), IRType::HttpResponse|IRType::JsonValue => "i64".into(), _ => "i64".into() } }
fn instr_llvm_type(instr: &Instr) -> String {
    match instr {
        Instr::Cmp(..) => "i1".into(),
        Instr::ConstBool(_) => "i1".into(),
        Instr::UnOp(op, _) => match op { crate::ast::UnOp::Not => "i1", _ => "i64" }.into(),
        Instr::BinOp(op, _, _) => match op { crate::ast::BinOp::And | crate::ast::BinOp::Or => "i1", _ => "i64" }.into(),
        Instr::ConstFloat(_) => "double".into(),
        Instr::IntrinsicCall { ret_ty, .. } => llvm_type(ret_ty),
        Instr::ExternCall { ret_ty, .. } => llvm_type(ret_ty),
        _ => "i64".into(),
    }
}
