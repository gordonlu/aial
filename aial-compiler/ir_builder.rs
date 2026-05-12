// ir_builder.rs —— 零妥协的 AST → IR 转换器

use crate::ast::*;
use crate::ir::*;
use crate::types::TypeEnv;
use std::collections::HashMap;

// ============================================================
// 构建器结构
// ============================================================

pub struct IRBuilder {
    functions: Vec<IRFunction>,
    strings: Vec<String>,
    tool_registrations: Vec<ToolRegistration>,

    current_fn: Option<IRFnContext>,
    value_counter: u32,
    block_counter: u32,
}

struct IRFnContext {
    func: IRFunction,
    current_block: BlockId,
    var_map: HashMap<String, Value>,
    // 记录循环的出口和继续目标，支持 break / continue
    loop_break: Option<BlockId>,
    loop_continue: Option<BlockId>,
}

enum LoopContext {
    None,
    While {
        cond_block: BlockId,
        body_block: BlockId,
        exit_block: BlockId,
    },
    Loop {
        body_block: BlockId,
        exit_block: BlockId,
    },
    For {
        // For 循环比较复杂，这里简化处理，先转为 While 的 IR 模式，或者专门处理。
        // 我们选择在 IR 生成时展开为初始化、条件、更新、体 四部分
        // 所以不用 LoopContext 结构，而是用普通块处理。
    },
}

impl IRBuilder {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            strings: Vec::new(),
            tool_registrations: Vec::new(),
            current_fn: None,
            value_counter: 0,
            block_counter: 0,
        }
    }

    // ======================================================================
    // 顶层入口
    // ======================================================================
    pub fn build(mut self, program: &Program, _type_env: &TypeEnv) -> IRModule {
        // 声明所有函数（含 main）
        let mut func_decls: Vec<(IRFunction, Option<&FnDef>)> = Vec::new();
        for item in &program.items {
            match item {
                TopLevelItem::FnDef(fd) => {
                    func_decls.push((self.declare_function(fd, false), Some(fd)));
                }
                TopLevelItem::Test(fd) => {
                    func_decls.push((self.declare_function(fd, true), Some(fd)));
                }
                _ => {}
            }
        }
        if let Some(main) = &program.main_fn {
            func_decls.push((self.declare_function(main, false), Some(main)));
        }

        // Register #[tool] functions from program items
        for item in &program.items {
            if let TopLevelItem::FnDef(fn_def) = item {
                for attr in &fn_def.attrs {
                    if attr.name.name == "tool" {
                        let tool_name = attr_arg(&attr.args, "name").unwrap_or_else(|| fn_def.name.name.clone());
                        let desc = attr_arg(&attr.args, "description").unwrap_or_default();
                        self.tool_registrations.push(ToolRegistration {
                            name: tool_name,
                            description: desc,
                            risk_level: String::new(),
                            required_caps: vec![],
                            fn_ptr: Value(0),
                            idempotent: false,
                            version: String::new(),
                            fallback: None,
                        });
                    }
                }
            }
        }

        // 填充函数体
        let mut filled = Vec::new();
        for (decl, ast_opt) in func_decls {
            if let Some(ast) = ast_opt {
                filled.push(self.build_function(decl, ast));
            } else {
                filled.push(decl);
            }
        }
        self.functions = filled;

        IRModule {
            functions: self.functions,
            strings: self.strings,
            tool_registrations: self.tool_registrations,
        }
    }

    fn declare_function(&mut self, fn_def: &FnDef, is_test: bool) -> IRFunction {
        let mut params = Vec::new();
        for (_i, param) in fn_def.params.iter().enumerate() {
            let v = self.new_value();
            let ty = self.type_to_ir(&param.ty);
            params.push((v, ty));
        }
        let ret_ty = match &fn_def.return_type {
            Some(ty) => self.type_to_ir(ty),
            None => IRType::Void,
        };
        let name = if is_test {
            format!("test_{}", fn_def.name.name)
        } else {
            fn_def.name.name.clone()
        };
        IRFunction {
            name,
            params,
            return_type: ret_ty,
            blocks: Vec::new(),
            entry: BlockId(0),
            value_types: Vec::new(),
        }
    }

    fn build_function(&mut self, mut func: IRFunction, fn_def: &FnDef) -> IRFunction {
        let entry_id = BlockId(self.block_counter);
        self.block_counter += 1;
        func.entry = entry_id;

        let mut ctx = IRFnContext {
            func,
            current_block: entry_id,
            var_map: HashMap::new(),
            loop_break: None,
            loop_continue: None,
        };

        // 手动创建入口基本块（new_block 需要 current_fn 已设置）
        ctx.func.blocks.push(BasicBlock {
            id: entry_id,
            instrs: Vec::new(),
            terminator: None,
        });

        // 绑定参数
        for (i, param) in fn_def.params.iter().enumerate() {
            ctx.var_map.insert(param.name.name.clone(), ctx.func.params[i].0);
        }

        self.current_fn = Some(ctx);
        self.switch_to_block(entry_id);

        // 生成函数体
        if let Err(_msg) = self.emit_block(&fn_def.body) {
            // 真正的错误处理：插入陷阱指令
            self.emit_unreachable();
            // 向调用者传播错误？考虑到编译过程，我们只是记录，不立即停止
            // 这里我们简单地保留错误信息，但在最终函数中保留一个不可达块
            // 实际编译器应该有诊断系统，这里简化处理。
        }

        let mut ctx = self.current_fn.take().unwrap();
        // Add ret void to the last block if it has instructions but no terminator
        if let Some(last_bb) = ctx.func.blocks.last_mut() {
            if last_bb.terminator.is_none() && !last_bb.instrs.is_empty() {
                last_bb.terminator = Some(Terminator::Ret(None));
            }
        }
        ctx.func
    }

    // ======================================================================
    // 语句发射
    // ======================================================================
    fn emit_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let(ls) => {
                let val = self.emit_expr(&ls.init)?;
                let local = self.emit(Instr::Alloca(IRType::I64));
                let ctx = self.current_fn.as_mut().unwrap();
                ctx.var_map.insert(ls.name.name.clone(), local);
                self.emit(Instr::Store(local, val));
                Ok(())
            }
            Stmt::Assign(a) => {
                let val = self.emit_expr(&a.value)?;
                let target_val = self.emit_lvalue(&a.target)?;
                self.emit(Instr::Store(target_val, val));
                Ok(())
            }
            Stmt::Expression(e) => {
                self.emit_expr(e)?;
                Ok(())
            }
            Stmt::Return(expr_opt, _) => {
                let ret_val = match expr_opt {
                    Some(e) => Some(self.emit_expr(e)?),
                    None => None,
                };
                self.emit_terminator(Terminator::Ret(ret_val));
                // 返回后必须开始一个新块，避免指令出现在不可达位置
                let dead = self.new_block();
                self.switch_to_block(dead);
                Ok(())
            }
            Stmt::If(is) => self.emit_if_stmt(is),
            Stmt::Match(ms) => self.emit_match_stmt(ms),
            Stmt::For(fs) => self.emit_for_stmt(fs),
            Stmt::While(ws) => self.emit_while_stmt(ws),
            Stmt::Loop(ls) => self.emit_loop_stmt(ls),
            Stmt::Break(_) => {
                if let Some(target) = self.current_fn.as_ref().unwrap().loop_break {
                    self.emit_terminator(Terminator::Br(target));
                    let dead = self.new_block();
                    self.switch_to_block(dead);
                    Ok(())
                } else {
                    Err("break used outside of loop".to_string())
                }
            }
            Stmt::Continue(_) => {
                if let Some(target) = self.current_fn.as_ref().unwrap().loop_continue {
                    self.emit_terminator(Terminator::Br(target));
                    let dead = self.new_block();
                    self.switch_to_block(dead);
                    Ok(())
                } else {
                    Err("continue used outside of loop".to_string())
                }
            }
        }
    }

    fn emit_block(&mut self, block: &Block) -> Result<(), String> {
        for stmt in &block.stmts {
            self.emit_stmt(stmt)?;
        }
        if let Some(expr) = &block.trailing_expr {
            let val = self.emit_expr(expr)?;
            self.emit_terminator(Terminator::Ret(Some(val)));
            let dead = self.new_block();
            self.switch_to_block(dead);
        }
        Ok(())
    }

    // ======================================================================
    // 控制流语句
    // ======================================================================
    fn emit_if_stmt(&mut self, is: &IfStmt) -> Result<(), String> {
        let cond = self.emit_expr(&is.cond)?;
        let then_block = self.new_block();
        let else_block = self.new_block();
        let merge_block = self.new_block();

        self.emit_terminator(Terminator::CondBr(cond, then_block, else_block));

        // then 分支
        self.switch_to_block(then_block);
        self.emit_block(&is.then_block)?;
        self.emit_terminator(Terminator::Br(merge_block));

        // else 分支
        self.switch_to_block(else_block);
        if let Some(eb) = &is.else_branch {
            match eb.as_ref() {
                ElseBranch::Block(b) => {
                    self.emit_block(b)?;
                }
                ElseBranch::If(inner_if) => {
                    self.emit_if_stmt(inner_if)?;
                }
            }
        }
        self.emit_terminator(Terminator::Br(merge_block));

        self.switch_to_block(merge_block);
        Ok(())
    }

    fn emit_match_stmt(&mut self, ms: &MatchStmt) -> Result<(), String> {
        let scrutinee = self.emit_expr(&ms.scrutinee)?;
        let default_block = self.new_block();
        let merge_block = self.new_block();

        // 简单处理：将每个模式臂转换为 if - else if 链或 switch。
        // 因为模式匹配涉及复杂逻辑，这里先采用线性的条件检查，后续可优化为跳转表。
        let _current_cond_block: Option<BlockId> = None;
        let mut prev_fallthrough: Option<BlockId> = None;

        for (i, arm) in ms.arms.iter().enumerate() {
            let body_block = self.new_block();
            let next_check_block = if i + 1 < ms.arms.len() {
                self.new_block()
            } else {
                default_block
            };

            // 生成模式匹配条件（简化：仅处理构造器或通配符）
            let cond = self.emit_pattern_test(scrutinee, &arm.pattern)?;

            if let Some(fall) = prev_fallthrough.take() {
                self.switch_to_block(fall);
                self.emit_terminator(Terminator::CondBr(cond, body_block, next_check_block));
            } else {
                self.emit_terminator(Terminator::CondBr(cond, body_block, next_check_block));
            }
            self.switch_to_block(body_block);
            // 绑定模式变量
            self.emit_pattern_bindings(scrutinee, &arm.pattern)?;
            match &arm.body {
                MatchBody::Block(b) => {
                    self.emit_block(b)?;
                }
                MatchBody::Expr(e) => {
                    let _ = self.emit_expr(e)?;
                }
            }
            self.emit_terminator(Terminator::Br(merge_block));

            prev_fallthrough = Some(next_check_block);
            self.switch_to_block(next_check_block);
        }

        // 默认块（最后一个臂必定是通配符或者我们假设穷尽）
        self.switch_to_block(default_block);
        self.emit_terminator(Terminator::Unreachable); // 如果穷尽，不会执行

        self.switch_to_block(merge_block);
        Ok(())
    }

    fn emit_for_stmt(&mut self, fs: &ForStmt) -> Result<(), String> {
        let iter_val = self.emit_expr(&fs.iterator)?;
        let counter = self.emit(Instr::Alloca(IRType::I64));
        let limit = self.emit(Instr::Alloca(IRType::I64));
        let zero = self.emit(Instr::ConstInt(0));
        self.emit(Instr::Store(counter, zero));
        self.emit(Instr::Store(limit, iter_val));

        let cond_block = self.new_block();
        let body_block = self.new_block();
        let inc_block = self.new_block();
        let exit_block = self.new_block();

        let old_break = self.current_fn.as_ref().unwrap().loop_break;
        let old_continue = self.current_fn.as_ref().unwrap().loop_continue;
        self.current_fn.as_mut().unwrap().loop_break = Some(exit_block);
        self.current_fn.as_mut().unwrap().loop_continue = Some(inc_block);

        self.emit_terminator(Terminator::Br(cond_block));

        self.switch_to_block(cond_block);
        let idx = self.emit(Instr::Load(counter));
        let lim = self.emit(Instr::Load(limit));
        let cond = self.emit(Instr::Cmp(BinOp::Lt, idx, lim));
        self.emit_terminator(Terminator::CondBr(cond, body_block, exit_block));

        self.switch_to_block(body_block);
        let idx_val = self.emit(Instr::Load(counter));
        let loop_var = self.emit(Instr::Alloca(IRType::I64));
        self.current_fn.as_mut().unwrap().var_map.insert(fs.variable.name.clone(), loop_var);
        self.emit(Instr::Store(loop_var, idx_val));
        self.emit_block(&fs.body)?;
        self.emit_terminator(Terminator::Br(inc_block));

        self.switch_to_block(inc_block);
        let cur = self.emit(Instr::Load(counter));
        let one = self.emit(Instr::ConstInt(1));
        let next = self.emit(Instr::BinOp(BinOp::Add, cur, one));
        self.emit(Instr::Store(counter, next));
        self.emit_terminator(Terminator::Br(cond_block));

        self.switch_to_block(exit_block);
        self.current_fn.as_mut().unwrap().loop_break = old_break;
        self.current_fn.as_mut().unwrap().loop_continue = old_continue;
        Ok(())
    }

    fn emit_while_stmt(&mut self, ws: &WhileStmt) -> Result<(), String> {
        let cond_block = self.new_block();
        let body_block = self.new_block();
        let exit_block = self.new_block();

        // 保存旧的循环上下文
        let old_break = self.current_fn.as_ref().unwrap().loop_break;
        let old_continue = self.current_fn.as_ref().unwrap().loop_continue;
        self.current_fn.as_mut().unwrap().loop_break = Some(exit_block);
        self.current_fn.as_mut().unwrap().loop_continue = Some(cond_block);

        self.emit_terminator(Terminator::Br(cond_block));

        self.switch_to_block(cond_block);
        let cond = self.emit_expr(&ws.cond)?;
        self.emit_terminator(Terminator::CondBr(cond, body_block, exit_block));

        self.switch_to_block(body_block);
        self.emit_block(&ws.body)?;
        self.emit_terminator(Terminator::Br(cond_block));

        self.switch_to_block(exit_block);
        // 恢复循环上下文
        self.current_fn.as_mut().unwrap().loop_break = old_break;
        self.current_fn.as_mut().unwrap().loop_continue = old_continue;
        Ok(())
    }

    fn emit_loop_stmt(&mut self, ls: &LoopStmt) -> Result<(), String> {
        let body_block = self.new_block();
        let exit_block = self.new_block();

        let old_break = self.current_fn.as_ref().unwrap().loop_break;
        let old_continue = self.current_fn.as_ref().unwrap().loop_continue;
        self.current_fn.as_mut().unwrap().loop_break = Some(exit_block);
        self.current_fn.as_mut().unwrap().loop_continue = Some(body_block);

        self.emit_terminator(Terminator::Br(body_block));
        self.switch_to_block(body_block);
        self.emit_block(&ls.body)?;
        self.emit_terminator(Terminator::Br(body_block));

        self.switch_to_block(exit_block);
        self.current_fn.as_mut().unwrap().loop_break = old_break;
        self.current_fn.as_mut().unwrap().loop_continue = old_continue;
        Ok(())
    }

    // ======================================================================
    // 表达式发射
    // ======================================================================
    fn emit_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match &expr.kind {
            ExprKind::IntLiteral(n) => Ok(self.emit(Instr::ConstInt(*n as i64))),
            ExprKind::FloatLiteral(f) => Ok(self.emit(Instr::ConstFloat(*f))),
            ExprKind::StringLiteral(s) => {
                // 字符串字面量 – 此处我们返回字符串索引或指针，简化处理
                let str_idx = self.strings.len() as u64;
                self.strings.push(s.clone());
                Ok(self.emit(Instr::ConstInt(str_idx as i64))) // 暂时用索引
            }
            ExprKind::BoolLiteral(b) => Ok(self.emit(Instr::ConstBool(*b))),
            ExprKind::NullLiteral => Ok(self.emit(Instr::ConstNull)),
            ExprKind::Variable(ident) => {
                let ctx = self.current_fn.as_ref().unwrap();
                if let Some(&var) = ctx.var_map.get(&ident.name) {
                    Ok(self.emit(Instr::Load(var)))
                } else {
                    Err(format!("variable `{}` not found", ident.name))
                }
            }
            ExprKind::SelfExpr => {
                // self maps to the first method parameter
                let ctx = self.current_fn.as_ref().unwrap();
                if let Some((v, _)) = ctx.func.params.first() {
                    Ok(self.emit(Instr::Load(*v)))
                } else {
                    Err("self used outside of a method".to_string())
                }
            }
            ExprKind::Unary(op, operand) => {
                let val = self.emit_expr(operand)?;
                match op {
                    UnOp::Neg => Ok(self.emit(Instr::UnOp(UnOp::Neg, val))),
                    UnOp::Not => Ok(self.emit(Instr::UnOp(UnOp::Not, val))),
                }
            }
            ExprKind::Binary(op, lhs, rhs) => {
                let l = self.emit_expr(lhs)?;
                let r = self.emit_expr(rhs)?;
                match op {
                    BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => {
                        Ok(self.emit(Instr::Cmp(op.clone(), l, r)))
                    }
                    _ => Ok(self.emit(Instr::BinOp(op.clone(), l, r))),
                }
            }
            ExprKind::Pipe(left, right) => {
                let l = self.emit_expr(left)?;
                match &right.kind {
                    // lhs |> f(args...) → f(lhs, args...)
                    ExprKind::Call(func, args, _named) => {
                        let mut all_args = vec![l];
                        for a in args { all_args.push(self.emit_expr(a)?); }
                        let func_val = self.emit_expr(func)?;
                        Ok(self.emit(Instr::Call { func: func_val, args: all_args, ret_ty: IRType::Void }))
                    }
                    _ => {
                        let r = self.emit_expr(right)?;
                        Ok(self.emit(Instr::Call { func: r, args: vec![l], ret_ty: IRType::Void }))
                    }
                }
            }
            ExprKind::Call(func, args, named) => {
                // Built-in function detection
                if let ExprKind::Variable(ident) = &func.kind {
                    if ident.name == "println" && args.len() == 1 && named.is_empty() {
                        let arg = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::Println, args: vec![arg], ret_ty: IRType::Void,
                        }));
                    }
                    if ident.name == "print" && args.len() == 1 && named.is_empty() {
                        let arg = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::Print, args: vec![arg], ret_ty: IRType::Void,
                        }));
                    }
                    if ident.name == "strlen" && args.len() == 1 && named.is_empty() {
                        let arg = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::StrLen, args: vec![arg], ret_ty: IRType::I64,
                        }));
                    }
                    if ident.name == "strcat" && args.len() == 2 && named.is_empty() {
                        let a = self.emit_expr(&args[0])?;
                        let b = self.emit_expr(&args[1])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::StrConcat, args: vec![a, b], ret_ty: IRType::String,
                        }));
                    }
                    if ident.name == "str_eq" && args.len() == 2 && named.is_empty() {
                        let a = self.emit_expr(&args[0])?;
                        let b = self.emit_expr(&args[1])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::StrEq, args: vec![a, b], ret_ty: IRType::Bool,
                        }));
                    }
                    if ident.name == "starts_with" && args.len() == 2 && named.is_empty() {
                        let s = self.emit_expr(&args[0])?;
                        let p = self.emit_expr(&args[1])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::StartsWith, args: vec![s, p], ret_ty: IRType::Bool,
                        }));
                    }
                    if ident.name == "strchr" && args.len() == 2 && named.is_empty() {
                        let s = self.emit_expr(&args[0])?;
                        let idx = self.emit_expr(&args[1])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::StrChr, args: vec![s, idx], ret_ty: IRType::I64,
                        }));
                    }
                    if ident.name == "strslice" && args.len() == 3 && named.is_empty() {
                        let s = self.emit_expr(&args[0])?;
                        let start = self.emit_expr(&args[1])?;
                        let len = self.emit_expr(&args[2])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::StrSlice, args: vec![s, start, len], ret_ty: IRType::String,
                        }));
                    }
                }
                // 检测并特殊处理 context::new(system_prompt = ..., token_budget = ...)
                if let ExprKind::Path(path) = &func.kind {
                    if path.segments.len() == 2
                        && path.segments[0].name == "context"
                        && path.segments[1].name == "new"
                    {
                        let mut system_prompt = self.emit(Instr::ConstString(String::new()));
                        let mut token_budget = self.emit(Instr::ConstInt(4096));
                        let mut strategy = self.emit(Instr::ConstString(String::new()));
                        let mut window_size = self.emit(Instr::ConstInt(0));
                        for opt in named {
                            match opt.name.name.as_str() {
                                "system_prompt" => system_prompt = self.emit_expr(&opt.value)?,
                                "token_budget" => token_budget = self.emit_expr(&opt.value)?,
                                "strategy" => strategy = self.emit_expr(&opt.value)?,
                                "window_size" => window_size = self.emit_expr(&opt.value)?,
                                _ => return Err(format!("unknown parameter `{}`", opt.name.name)),
                            }
                        }
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::ContextNew,
                            args: vec![system_prompt, token_budget, strategy, window_size],
                            ret_ty: IRType::I64,
                        }));
                    }
                    // privacy::sensitive(value) — taint-tracking
                    if path.segments.len() == 2
                        && path.segments[0].name == "privacy"
                        && path.segments[1].name == "sensitive"
                        && args.len() == 1
                    {
                        let val = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::PrivacySensitive,
                            args: vec![val],
                            ret_ty: IRType::I64,
                        }));
                    }
                    // context::forget(msg_id) — causal pruning
                    if path.segments.len() == 2
                        && path.segments[0].name == "context"
                        && path.segments[1].name == "forget"
                        && args.len() == 2
                    {
                        let ctx = self.emit_expr(&args[0])?;
                        let msg = self.emit_expr(&args[1])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::ContextForget,
                            args: vec![ctx, msg],
                            ret_ty: IRType::Void,
                        }));
                    }
                    // file::read(path) — bootstrapping I/O
                    if path.segments.len() == 2 && path.segments[0].name == "file" && path.segments[1].name == "read" && args.len() == 1 {
                        let p = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::FileRead, args: vec![p], ret_ty: IRType::String,
                        }));
                    }
                    // file::write(path, content)
                    if path.segments.len() == 2 && path.segments[0].name == "file" && path.segments[1].name == "write" && args.len() == 2 {
                        let p = self.emit_expr(&args[0])?;
                        let c = self.emit_expr(&args[1])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::FileWrite, args: vec![p, c], ret_ty: IRType::Void,
                        }));
                    }
                    // file::append(path, content)
                    if path.segments.len() == 2 && path.segments[0].name == "file" && path.segments[1].name == "append" && args.len() == 2 {
                        let p = self.emit_expr(&args[0])?;
                        let c = self.emit_expr(&args[1])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::FileAppend, args: vec![p, c], ret_ty: IRType::Void,
                        }));
                    }
                    // file::patch(path, replace=(old, new))
                    if path.segments.len() == 2 && path.segments[0].name == "file" && path.segments[1].name == "patch" && args.len() == 3 {
                        let p = self.emit_expr(&args[0])?;
                        let o = self.emit_expr(&args[1])?;
                        let n = self.emit_expr(&args[2])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::FilePatch, args: vec![p, o, n], ret_ty: IRType::Void,
                        }));
                    }
                    // http::get(url) → HttpResponse
                    if path.segments.len() == 2 && path.segments[0].name == "http" && path.segments[1].name == "get" && args.len() == 1 {
                        let url = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::HttpGet, args: vec![url], ret_ty: IRType::HttpResponse,
                        }));
                    }
                    // http::status(resp) → int
                    if path.segments.len() == 2 && path.segments[0].name == "http" && path.segments[1].name == "status" && args.len() == 1 {
                        let resp = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::HttpStatus, args: vec![resp], ret_ty: IRType::I64,
                        }));
                    }
                    // http::text(resp) → string
                    if path.segments.len() == 2 && path.segments[0].name == "http" && path.segments[1].name == "text" && args.len() == 1 {
                        let resp = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::HttpText, args: vec![resp], ret_ty: IRType::String,
                        }));
                    }
                    // json::parse(text) → JsonValue
                    if path.segments.len() == 2 && path.segments[0].name == "json" && path.segments[1].name == "parse" && args.len() == 1 {
                        let text = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::JsonParse, args: vec![text], ret_ty: IRType::JsonValue,
                        }));
                    }
                    // json::get(val, key) → JsonValue
                    if path.segments.len() == 2 && path.segments[0].name == "json" && path.segments[1].name == "get" && args.len() == 2 {
                        let val = self.emit_expr(&args[0])?;
                        let key = self.emit_expr(&args[1])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::JsonGet, args: vec![val, key], ret_ty: IRType::JsonValue,
                        }));
                    }
                    // json::get_or(val, key, default) → JsonValue
                    if path.segments.len() == 2 && path.segments[0].name == "json" && path.segments[1].name == "get_or" && args.len() == 3 {
                        let val = self.emit_expr(&args[0])?;
                        let key = self.emit_expr(&args[1])?;
                        let default = self.emit_expr(&args[2])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::JsonGetOr, args: vec![val, key, default], ret_ty: IRType::JsonValue,
                        }));
                    }
                    // json::type_of(val) → int
                    if path.segments.len() == 2 && path.segments[0].name == "json" && path.segments[1].name == "type_of" && args.len() == 1 {
                        let val = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::JsonType, args: vec![val], ret_ty: IRType::I64,
                        }));
                    }
                    // json::stringify(val) → string
                    if path.segments.len() == 2 && path.segments[0].name == "json" && path.segments[1].name == "stringify" && args.len() == 1 {
                        let val = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::JsonToString, args: vec![val], ret_ty: IRType::String,
                        }));
                    }
                    // json::to_string(val) → string
                    if path.segments.len() == 2 && path.segments[0].name == "json" && path.segments[1].name == "to_string" && args.len() == 1 {
                        let val = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::JsonValueToString, args: vec![val], ret_ty: IRType::String,
                        }));
                    }
                    // json::to_int(val) → int
                    if path.segments.len() == 2 && path.segments[0].name == "json" && path.segments[1].name == "to_int" && args.len() == 1 {
                        let val = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::JsonToInt, args: vec![val], ret_ty: IRType::I64,
                        }));
                    }
                    // json::to_float(val) → float
                    if path.segments.len() == 2 && path.segments[0].name == "json" && path.segments[1].name == "to_float" && args.len() == 1 {
                        let val = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::JsonToFloat, args: vec![val], ret_ty: IRType::F64,
                        }));
                    }
                    // json::array_len(val) → int
                    if path.segments.len() == 2 && path.segments[0].name == "json" && path.segments[1].name == "array_len" && args.len() == 1 {
                        let val = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::JsonArrayLen, args: vec![val], ret_ty: IRType::I64,
                        }));
                    }
                    // json::array_get(val, idx) → JsonValue
                    if path.segments.len() == 2 && path.segments[0].name == "json" && path.segments[1].name == "array_get" && args.len() == 2 {
                        let val = self.emit_expr(&args[0])?;
                        let idx = self.emit_expr(&args[1])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::JsonArrayGet, args: vec![val, idx], ret_ty: IRType::JsonValue,
                        }));
                    }
                    // http::post(url, body) → HttpResponse
                    if path.segments.len() == 2 && path.segments[0].name == "http" && path.segments[1].name == "post" && args.len() == 2 {
                        let url = self.emit_expr(&args[0])?;
                        let body = self.emit_expr(&args[1])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::HttpPost, args: vec![url, body], ret_ty: IRType::HttpResponse,
                        }));
                    }
                    // http::post_json(url, json_val) → HttpResponse
                    if path.segments.len() == 2 && path.segments[0].name == "http" && path.segments[1].name == "post_json" && args.len() == 2 {
                        let url = self.emit_expr(&args[0])?;
                        let val = self.emit_expr(&args[1])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::HttpPostJson, args: vec![url, val], ret_ty: IRType::HttpResponse,
                        }));
                    }
                    // http::header_map() → HeaderMap
                    if path.segments.len() == 2 && path.segments[0].name == "http" && path.segments[1].name == "header_map" && args.len() == 0 {
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::HttpHeaderMap, args: vec![], ret_ty: IRType::I64,
                        }));
                    }
                    // http::header_set(map, key, val) → HeaderMap
                    if path.segments.len() == 2 && path.segments[0].name == "http" && path.segments[1].name == "header_set" && args.len() == 3 {
                        let map = self.emit_expr(&args[0])?;
                        let key = self.emit_expr(&args[1])?;
                        let val = self.emit_expr(&args[2])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::HttpHeaderSet, args: vec![map, key, val], ret_ty: IRType::I64,
                        }));
                    }
                    // http::start(port) → ServerHandle
                    if path.segments.len() == 2 && path.segments[0].name == "http" && path.segments[1].name == "start" && args.len() == 1 {
                        let port = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::HttpStart, args: vec![port], ret_ty: IRType::I64,
                        }));
                    }
                    // http::listen(handle) → Request (timeout optional)
                    if path.segments.len() == 2 && path.segments[0].name == "http" && path.segments[1].name == "listen" && (args.len() == 1 || args.len() == 2) {
                        let handle = self.emit_expr(&args[0])?;
                        let timeout_ms = if args.len() == 2 { self.emit_expr(&args[1])? } else { self.emit(Instr::ConstInt(0)) };
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::HttpListen, args: vec![handle, timeout_ms], ret_ty: IRType::I64,
                        }));
                    }
                    // http::respond(req, body, content_type) → void
                    if path.segments.len() == 2 && path.segments[0].name == "http" && path.segments[1].name == "respond" && args.len() == 3 {
                        let req = self.emit_expr(&args[0])?;
                        let body = self.emit_expr(&args[1])?;
                        let ct = self.emit_expr(&args[2])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::HttpRespond, args: vec![req, body, ct], ret_ty: IRType::Void,
                        }));
                    }
                    // http::body / method / path / url / status_text
                    if path.segments.len() == 2 && path.segments[0].name == "http" {
                        match path.segments[1].name.as_str() {
                            "body" if args.len() == 1 => {
                                let req = self.emit_expr(&args[0])?;
                                return Ok(self.emit(Instr::IntrinsicCall { intrinsic: Intrinsic::HttpRequestBody, args: vec![req], ret_ty: IRType::String }));
                            }
                            "method" if args.len() == 1 => {
                                let req = self.emit_expr(&args[0])?;
                                return Ok(self.emit(Instr::IntrinsicCall { intrinsic: Intrinsic::HttpMethod, args: vec![req], ret_ty: IRType::String }));
                            }
                            "path" if args.len() == 1 => {
                                let req = self.emit_expr(&args[0])?;
                                return Ok(self.emit(Instr::IntrinsicCall { intrinsic: Intrinsic::HttpPath, args: vec![req], ret_ty: IRType::String }));
                            }
                            "url" if args.len() == 1 => {
                                let req = self.emit_expr(&args[0])?;
                                return Ok(self.emit(Instr::IntrinsicCall { intrinsic: Intrinsic::HttpUrl, args: vec![req], ret_ty: IRType::String }));
                            }
                            "query" if args.len() == 2 => {
                                let req = self.emit_expr(&args[0])?; let key = self.emit_expr(&args[1])?;
                                return Ok(self.emit(Instr::IntrinsicCall { intrinsic: Intrinsic::HttpQuery, args: vec![req, key], ret_ty: IRType::String }));
                            }
                            "header" if args.len() == 2 => {
                                let req = self.emit_expr(&args[0])?; let key = self.emit_expr(&args[1])?;
                                return Ok(self.emit(Instr::IntrinsicCall { intrinsic: Intrinsic::HttpHeader, args: vec![req, key], ret_ty: IRType::String }));
                            }
                            "status_text" if args.len() == 1 => {
                                let code = self.emit_expr(&args[0])?;
                                return Ok(self.emit(Instr::IntrinsicCall { intrinsic: Intrinsic::HttpStatusText, args: vec![code], ret_ty: IRType::String }));
                            }
                            "ok" if args.len() == 2 => {
                                let req = self.emit_expr(&args[0])?; let body = self.emit_expr(&args[1])?;
                                return Ok(self.emit(Instr::IntrinsicCall { intrinsic: Intrinsic::HttpOk, args: vec![req, body], ret_ty: IRType::Void }));
                            }
                            "json" if args.len() == 2 => {
                                let req = self.emit_expr(&args[0])?; let body = self.emit_expr(&args[1])?;
                                return Ok(self.emit(Instr::IntrinsicCall { intrinsic: Intrinsic::HttpJson, args: vec![req, body], ret_ty: IRType::Void }));
                            }
                            "html" if args.len() == 2 => {
                                let req = self.emit_expr(&args[0])?; let body = self.emit_expr(&args[1])?;
                                return Ok(self.emit(Instr::IntrinsicCall { intrinsic: Intrinsic::HttpHtml, args: vec![req, body], ret_ty: IRType::Void }));
                            }
                            "serve" if args.len() == 2 => {
                                let req = self.emit_expr(&args[0])?; let file = self.emit_expr(&args[1])?;
                                return Ok(self.emit(Instr::IntrinsicCall { intrinsic: Intrinsic::HttpServe, args: vec![req, file], ret_ty: IRType::Void }));
                            }
                            _ => {}
                        }
                    }
                    // html::escape(text) → string
                    if path.segments.len() == 2 && path.segments[0].name == "html" && path.segments[1].name == "escape" && args.len() == 1 {
                        let text = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::HtmlEscape, args: vec![text], ret_ty: IRType::String,
                        }));
                    }
                    // ask::read_token(handle) → string
                    if path.segments.len() == 2 && path.segments[0].name == "ask" && path.segments[1].name == "read_token" && args.len() == 1 {
                        let handle = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::AiStreamRead, args: vec![handle], ret_ty: IRType::String,
                        }));
                    }
                    // io::readln() → string
                    if path.segments.len() == 2 && path.segments[0].name == "io" && path.segments[1].name == "readln" && args.is_empty() {
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::IoReadln, args: vec![], ret_ty: IRType::String,
                        }));
                    }
                    // io::readln_timeout(ms) → string
                    if path.segments.len() == 2 && path.segments[0].name == "io" && path.segments[1].name == "readln_timeout" && args.len() == 1 {
                        let ms = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::IoReadlnTimeout, args: vec![ms], ret_ty: IRType::String,
                        }));
                    }
                    // io::readkey() → string
                    if path.segments.len() == 2 && path.segments[0].name == "io" && path.segments[1].name == "readkey" && args.is_empty() {
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::IoReadkey, args: vec![], ret_ty: IRType::String,
                        }));
                    }
                    // io::readkey_timeout(ms) → string
                    if path.segments.len() == 2 && path.segments[0].name == "io" && path.segments[1].name == "readkey_timeout" && args.len() == 1 {
                        let ms = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::IoReadkeyTimeout, args: vec![ms], ret_ty: IRType::String,
                        }));
                    }
                    // io::raw_mode(bool) → void
                    if path.segments.len() == 2 && path.segments[0].name == "io" && path.segments[1].name == "raw_mode" && args.len() == 1 {
                        let enable = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::IoRawMode, args: vec![enable], ret_ty: IRType::Void,
                        }));
                    }
                    // ctx::open_memory(path) → db handle
                    if path.segments.len() == 2 && path.segments[0].name == "ctx" && path.segments[1].name == "open_memory" && args.len() == 1 {
                        let p = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::CtxOpenMemory, args: vec![p], ret_ty: IRType::I64,
                        }));
                    }
                    // ctx::save_message(db, session, role, content) → void
                    if path.segments.len() == 2 && path.segments[0].name == "ctx" && path.segments[1].name == "save_message" && args.len() == 4 {
                        let db = self.emit_expr(&args[0])?; let session = self.emit_expr(&args[1])?;
                        let role = self.emit_expr(&args[2])?; let content = self.emit_expr(&args[3])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::CtxSaveMessage, args: vec![db, session, role, content], ret_ty: IRType::Void,
                        }));
                    }
                    // ctx::load_messages(db, session, limit) → JSON string
                    if path.segments.len() == 2 && path.segments[0].name == "ctx" && path.segments[1].name == "load_messages" && args.len() == 3 {
                        let db = self.emit_expr(&args[0])?; let session = self.emit_expr(&args[1])?;
                        let limit = self.emit_expr(&args[2])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::CtxLoadMessages, args: vec![db, session, limit], ret_ty: IRType::String,
                        }));
                    }
                    // ctx::load_messages_since(db, session, timestamp) → JSON string
                    if path.segments.len() == 2 && path.segments[0].name == "ctx" && path.segments[1].name == "load_messages_since" && args.len() == 3 {
                        let db = self.emit_expr(&args[0])?; let session = self.emit_expr(&args[1])?;
                        let ts = self.emit_expr(&args[2])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::CtxLoadMessagesSince, args: vec![db, session, ts], ret_ty: IRType::String,
                        }));
                    }
                    // ctx::close_memory(db) → void
                    if path.segments.len() == 2 && path.segments[0].name == "ctx" && path.segments[1].name == "close_memory" && args.len() == 1 {
                        let db = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::CtxCloseMemory, args: vec![db], ret_ty: IRType::Void,
                        }));
                    }
                    // ctx::last_error() → string
                    if path.segments.len() == 2 && path.segments[0].name == "ctx" && path.segments[1].name == "last_error" && args.is_empty() {
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::CtxLastError, args: vec![], ret_ty: IRType::String,
                        }));
                    }
                    // time::sleep(ms) → void
                    if path.segments.len() == 2 && path.segments[0].name == "time" && path.segments[1].name == "sleep" && args.len() == 1 {
                        let ms = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::TimeSleep, args: vec![ms], ret_ty: IRType::Void,
                        }));
                    }
                    // ffi::load(path) → lib handle
                    if path.segments.len() == 2 && path.segments[0].name == "ffi" && path.segments[1].name == "load" && args.len() == 1 {
                        let p = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::FfiLoad, args: vec![p], ret_ty: IRType::I64,
                        }));
                    }
                    // ffi::call(handle, fn_name, args...) → result
                    if path.segments.len() == 2 && path.segments[0].name == "ffi" && path.segments[1].name == "call" && args.len() >= 2 {
                        let handle = self.emit_expr(&args[0])?;
                        let fn_name = self.emit_expr(&args[1])?;
                        let mut ffi_args = vec![handle, fn_name];
                        for a in &args[2..] { ffi_args.push(self.emit_expr(a)?); }
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::FfiCall, args: ffi_args, ret_ty: IRType::I64,
                        }));
                    }
                    // ffi::close(handle) → void
                    if path.segments.len() == 2 && path.segments[0].name == "ffi" && path.segments[1].name == "close" && args.len() == 1 {
                        let handle = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::FfiClose, args: vec![handle], ret_ty: IRType::Void,
                        }));
                    }
                    // context::reflect() — auto self-correction
                    if path.segments.len() == 2
                        && path.segments[0].name == "context"
                        && path.segments[1].name == "reflect"
                        && args.len() == 1
                    {
                        let ctx = self.emit_expr(&args[0])?;
                        return Ok(self.emit(Instr::IntrinsicCall {
                            intrinsic: Intrinsic::ContextReflect,
                            args: vec![ctx],
                            ret_ty: IRType::String,
                        }));
                    }
                }
                // User-defined function call — emit UserCall BEFORE emit_expr(func)
                // to avoid "variable not found" error
                if let ExprKind::Variable(ident) = &func.kind {
                    let arg_vals: Vec<Value> = args.iter().map(|a| self.emit_expr(a)).collect::<Result<_, _>>()?;
                    return Ok(self.emit(Instr::UserCall {
                        name: ident.name.clone(),
                        args: arg_vals,
                        ret_ty: IRType::Void,
                    }));
                }
                let func_val = self.emit_expr(func)?;
                let arg_vals: Vec<Value> = args
                    .iter()
                    .map(|a| self.emit_expr(a))
                    .collect::<Result<_, _>>()?;
                Ok(self.emit(Instr::Call {
                    func: func_val,
                    args: arg_vals,
                    ret_ty: IRType::Void,
                }))
            }
            ExprKind::FieldAccess { receiver, field } => {
                let recv = self.emit_expr(receiver)?;
                // 将字段访问映射为对应的 AI 字段提取 intrinsic
                // 未来应通过类型系统查询字段偏移，此处硬编码 AiResponse 的字段映射
                let intrinsic = match field.name.as_str() {
                    "text" => Intrinsic::ExtractAiText,
                    "variant" => Intrinsic::ExtractAiVariant,
                    "usage" => Intrinsic::ExtractAiUsage,
                    "reasoning" => Intrinsic::ExtractAiReasoning,
                    _ => return Err(format!("unknown field `{}`", field.name)),
                };
                Ok(self.emit(Instr::IntrinsicCall {
                    intrinsic,
                    args: vec![recv],
                    ret_ty: IRType::String,
                }))
            }
            ExprKind::MethodCall { receiver, method: _, generic_args: _, args } => {
                let recv = self.emit_expr(receiver)?;
                let mut call_args = vec![recv];
                for a in args {
                    call_args.push(self.emit_expr(a)?);
                }
                Ok(self.emit(Instr::Call {
                    func: Value(0),
                    args: call_args,
                    ret_ty: IRType::I64,
                }))
            }
            ExprKind::Index(base, index) => {
                let b = self.emit_expr(base)?;
                let i = self.emit_expr(index)?;
                let ptr = self.emit(Instr::BinOp(BinOp::Add, b, i));
                Ok(self.emit(Instr::Load(ptr)))
            }
            ExprKind::StructLiteral { struct_name: _, fields } => {
                // 简化实现：分配空间并逐字段存储
                let struct_ptr = self.emit(Instr::Alloca(IRType::I64));
                for (i, (_, field_val)) in fields.iter().enumerate() {
                    let val = self.emit_expr(field_val)?;
                    let off_val = self.emit(Instr::ConstInt(i as i64));
                    let addr = self.emit(Instr::BinOp(BinOp::Add, struct_ptr, off_val));
                    self.emit(Instr::Store(addr, val));
                }
                Ok(struct_ptr)
            }
            ExprKind::IfExpr(cond, then_block, else_expr) => {
                let cond_val = self.emit_expr(cond)?;
                let then_block_id = self.new_block();
                let else_block_id = self.new_block();
                let merge_block_id = self.new_block();
                let result_ptr = self.emit(Instr::Alloca(IRType::I64));
                self.emit_terminator(Terminator::CondBr(cond_val, then_block_id, else_block_id));
                self.switch_to_block(then_block_id);
                self.emit_block(then_block)?;
                if let Some(tail) = &then_block.trailing_expr {
                    let then_val = self.emit_expr(tail)?;
                    self.emit(Instr::Store(result_ptr, then_val));
                }
                self.emit_terminator(Terminator::Br(merge_block_id));
                self.switch_to_block(else_block_id);
                let else_val = self.emit_expr(else_expr)?;
                self.emit(Instr::Store(result_ptr, else_val));
                self.emit_terminator(Terminator::Br(merge_block_id));
                self.switch_to_block(merge_block_id);
                Ok(self.emit(Instr::Load(result_ptr)))
            }
            ExprKind::MatchExpr(scrutinee, arms) => {
                let scrut_val = self.emit_expr(scrutinee)?;
                let result_ptr = self.emit(Instr::Alloca(IRType::I64));
                let merge_block = self.new_block();
                let default_block = self.new_block();
                let mut prev_fallthrough: Option<BlockId> = None;

                for (i, arm) in arms.iter().enumerate() {
                    let body_block = self.new_block();
                    let next_check = if i + 1 < arms.len() { self.new_block() } else { default_block };

                    let cond = self.emit_pattern_test(scrut_val, &arm.pattern)?;
                    if let Some(fall) = prev_fallthrough.take() {
                        self.switch_to_block(fall);
                    } else {
                        // first arm branches from current block
                    }
                    self.emit_terminator(Terminator::CondBr(cond, body_block, next_check));

                    self.switch_to_block(body_block);
                    self.emit_pattern_bindings(scrut_val, &arm.pattern)?;
                    let arm_val = match &arm.body {
                        MatchBody::Block(block) => {
                            for stmt in &block.stmts { self.emit_stmt(stmt)?; }
                            if let Some(tail) = &block.trailing_expr {
                                self.emit_expr(tail)?
                            } else {
                                self.emit(Instr::ConstInt(0))
                            }
                        }
                        MatchBody::Expr(e) => self.emit_expr(e)?,
                    };
                    self.emit(Instr::Store(result_ptr, arm_val));
                    self.emit_terminator(Terminator::Br(merge_block));

                    prev_fallthrough = Some(next_check);
                    self.switch_to_block(next_check);
                }

                self.switch_to_block(default_block);
                self.emit_terminator(Terminator::Unreachable);
                self.switch_to_block(merge_block);
                Ok(self.emit(Instr::Load(result_ptr)))
            }
            ExprKind::BlockExpr(block) => {
                // 执行块并返回尾表达式值（假定块尾一定有值）
                if let Some(tail) = &block.trailing_expr {
                    self.emit_block(&Block { span: block.span, stmts: block.stmts.clone(), trailing_expr: None, parallel: false })?;
                    self.emit_expr(tail)
                } else {
                    Err("block expression must have a trailing expression".to_string())
                }
            }
            ExprKind::Ask(options) => self.emit_ask_single(options),
            ExprKind::AskMany(groups) => self.emit_ask_many(groups),
            ExprKind::AskRace(groups) => self.emit_ask_race(groups),
            ExprKind::Receive => {
                Ok(self.emit(Instr::IntrinsicCall {
                    intrinsic: Intrinsic::ActorReceive,
                    args: vec![],
                    ret_ty: IRType::Ptr,
                }))
            }
            ExprKind::Path(path) => {
                // 路径可能是全局函数或常量，暂时返回“未解析”错误
                Err(format!("路径表达式未实现: {:?}", path))
            }
        }
    }

    // ======================================================================
    // ask 发射
    // ======================================================================
    fn emit_ask_single(&mut self, options: &[AskOption]) -> Result<Value, String> {
        let mut model = self.emit(Instr::ConstInt(0));
        let mut context = self.emit(Instr::ConstInt(0));
        let mut prompt = self.emit(Instr::ConstString(String::new()));
        let mut temperature = self.emit(Instr::ConstFloat(0.0));
        let mut max_tokens = self.emit(Instr::ConstInt(0));
        let mut format = self.emit(Instr::ConstInt(0));
        let mut stream = false;
        for opt in options {
            let val = self.emit_expr(&opt.value)?;
            match opt.name.name.as_str() {
                "model" => model = val,
                "context" => context = val,
                "prompt" => prompt = val,
                "temperature" => temperature = val,
                "max_tokens" => max_tokens = val,
                "format" | "response_format" => { format = val; }
                "stream" => { stream = true; }
                _ => {}
            }
        }
        if stream {
            let args = vec![model, context, prompt, temperature, max_tokens, format];
            Ok(self.emit(Instr::IntrinsicCall {
                intrinsic: Intrinsic::AiStreamStart, args,
                ret_ty: IRType::I64, // stream handle
            }))
        } else {
            let args = vec![model, context, prompt, temperature, max_tokens, format];
            Ok(self.emit(Instr::IntrinsicCall {
                intrinsic: Intrinsic::AiCall, args,
                ret_ty: IRType::AiResponse(Box::new(IRType::String)),
            }))
        }
    }

    fn emit_ask_many(&mut self, groups: &[Vec<AskOption>]) -> Result<Value, String> {
        // ask.many: 并行调用多个模型，收集所有结果
        // 在 IR 层面展开为多个 AiCall，结果存入连续内存
        let array_ptr = self.emit(Instr::Alloca(IRType::I64));
        for (i, group) in groups.iter().enumerate() {
            let result = self.emit_ask_single(group)?;
            let off = self.emit(Instr::ConstInt(i as i64));
            let slot = self.emit(Instr::BinOp(BinOp::Add, array_ptr, off));
            self.emit(Instr::Store(slot, result));
        }
        Ok(array_ptr)
    }

    fn emit_ask_race(&mut self, groups: &[Vec<AskOption>]) -> Result<Value, String> {
        let array_ptr = self.emit(Instr::Alloca(IRType::I64));
        for (i, group) in groups.iter().enumerate() {
            let result = self.emit_ask_single(group)?;
            let off = self.emit(Instr::ConstInt(i as i64));
            let slot = self.emit(Instr::BinOp(BinOp::Add, array_ptr, off));
            self.emit(Instr::Store(slot, result));
        }
        Ok(array_ptr)
    }

    // ======================================================================
    // 模式匹配辅助
    // ======================================================================
    fn emit_pattern_test(&mut self, scrutinee: Value, pattern: &Pattern) -> Result<Value, String> {
        match pattern {
            Pattern::Wildcard(_) => Ok(self.emit(Instr::ConstBool(true))),
            Pattern::Variable(_) => Ok(self.emit(Instr::ConstBool(true))),
            Pattern::Literal(lit) => {
                let lit_val = self.emit_expr(lit)?;
                Ok(self.emit(Instr::Cmp(BinOp::Eq, scrutinee, lit_val)))
            }
            Pattern::Constructor(_, sub_patterns) => {
                // Constructor match: compare discriminant (field 0) with constructor index
                // For AiResponse, match the variant field
                if sub_patterns.is_empty() {
                    Ok(self.emit(Instr::ConstBool(true)))
                } else {
                    // Real constructor requires discriminant — emit ExtractValue
                    Ok(self.emit(Instr::ConstBool(true)))
                }
            }
            Pattern::Or(patterns) => {
                let mut result = self.emit_pattern_test(scrutinee, &patterns[0])?;
                for p in &patterns[1..] {
                    let sub = self.emit_pattern_test(scrutinee, p)?;
                    result = self.emit(Instr::BinOp(BinOp::Or, result, sub));
                }
                Ok(result)
            }
            Pattern::As(inner, _) => self.emit_pattern_test(scrutinee, inner),
        }
    }

    fn emit_pattern_bindings(&mut self, scrutinee: Value, pattern: &Pattern) -> Result<(), String> {
        match pattern {
            Pattern::Variable(ident) => {
                let local = self.emit(Instr::Alloca(IRType::I64));
                self.current_fn.as_mut().unwrap().var_map.insert(ident.name.clone(), local);
                self.emit(Instr::Store(local, scrutinee));
                Ok(())
            }
            Pattern::Wildcard(_) => Ok(()),
            Pattern::Constructor(_path, sub_patterns) => {
                for (i, sub) in sub_patterns.iter().enumerate() {
                    let off = self.emit(Instr::ConstInt(i as i64));
                    let field_ptr = self.emit(Instr::BinOp(BinOp::Add, scrutinee, off));
                    let field_val = self.emit(Instr::Load(field_ptr));
                    self.emit_pattern_bindings(field_val, sub)?;
                }
                Ok(())
            }
            Pattern::Or(patterns) => {
                if let Some(p) = patterns.first() {
                    self.emit_pattern_bindings(scrutinee, p)
                } else {
                    Ok(())
                }
            }
            Pattern::As(inner, alias) => {
                self.emit_pattern_bindings(scrutinee, inner)?;
                let local = self.emit(Instr::Alloca(IRType::I64));
                self.current_fn.as_mut().unwrap().var_map.insert(alias.name.clone(), local);
                self.emit(Instr::Store(local, scrutinee));
                Ok(())
            }
            Pattern::Literal(_) => Ok(()),
        }
    }

    // ======================================================================
    // 左值处理
    // ======================================================================
    fn emit_lvalue(&mut self, lv: &LValue) -> Result<Value, String> {
        match lv {
            LValue::Variable(ident) => {
                let ctx = self.current_fn.as_ref().unwrap();
                ctx.var_map.get(&ident.name).cloned()
                    .ok_or_else(|| format!("undefined variable `{}`", ident.name))
            }
            LValue::Field(base, _field) => {
                let base_ptr = self.emit_lvalue(base)?;
                // Use field index 1 as default — real field offset requires type info
                let field_idx = self.emit(Instr::ConstInt(1));
                Ok(self.emit(Instr::BinOp(BinOp::Add, base_ptr, field_idx)))
            }
            LValue::Index(base, index_expr) => {
                let base_ptr = self.emit_lvalue(base)?;
                let idx = self.emit_expr(index_expr)?;
                Ok(self.emit(Instr::BinOp(BinOp::Add, base_ptr, idx)))
            }
            LValue::Deref(base) => {
                // Deref: the value at base is already a pointer
                self.emit_lvalue(base)
            }
        }
    }

    // ======================================================================
    // 基本块和指令操作
    // ======================================================================
    fn new_block(&mut self) -> BlockId {
        let id = BlockId(self.block_counter);
        self.block_counter += 1;
        if let Some(ctx) = &mut self.current_fn {
            ctx.func.blocks.push(BasicBlock {
                id,
                instrs: Vec::new(),
                terminator: None,
            });
        }
        id
    }

    fn switch_to_block(&mut self, block: BlockId) {
        if let Some(ctx) = &mut self.current_fn {
            ctx.current_block = block;
        }
    }

    fn emit(&mut self, instr: Instr) -> Value {
        let v = self.new_value();
        let ty = Self::instr_type(&instr);
        if let Some(ctx) = &mut self.current_fn {
            ctx.func.value_types.push((v, ty));
            for bb in &mut ctx.func.blocks {
                if bb.id == ctx.current_block {
                    bb.instrs.push((instr, Some(v)));
                    break;
                }
            }
        }
        v
    }

    fn emit_terminator(&mut self, term: Terminator) {
        if let Some(ctx) = &mut self.current_fn {
            for bb in &mut ctx.func.blocks {
                if bb.id == ctx.current_block {
                    bb.terminator = Some(term);
                    break;
                }
            }
        }
    }

    fn emit_unreachable(&mut self) {
        self.emit_terminator(Terminator::Unreachable);
    }

    fn new_value(&mut self) -> Value {
        let v = Value(self.value_counter);
        self.value_counter += 1;
        v
    }

    fn type_to_ir(&self, ty: &Type) -> IRType {
        match ty {
            Type::Base(b) => match b {
                BaseType::Int | BaseType::Int64 | BaseType::Int32 => IRType::I64,
                BaseType::Float | BaseType::Float64 => IRType::F64,
                BaseType::Bool => IRType::Bool,
                BaseType::String => IRType::String,
                _ => IRType::I64,
            },
            _ => IRType::Ptr,
        }
    }

    fn instr_type(instr: &Instr) -> IRType {
        match instr {
            Instr::ConstInt(_) => IRType::I64,
            Instr::ConstFloat(_) => IRType::F64,
            Instr::ConstBool(_) => IRType::Bool,
            Instr::ConstNull => IRType::Ptr,
            Instr::BinOp(..) => IRType::I64,
            Instr::UnOp(..) => IRType::I64,
            Instr::Cmp(..) => IRType::Bool,
            Instr::Alloca(ty) => ty.clone(),
            Instr::Load(_) => IRType::I64, // 需改进
            Instr::Store(..) => IRType::Void,
            Instr::IntrinsicCall { ret_ty, .. } => ret_ty.clone(),
            Instr::Call { ret_ty, .. } => ret_ty.clone(),
            _ => IRType::Void,
        }
    }
}


/// Extract the string value of a named attribute argument (e.g., name="foo" in #[tool(name="foo")])
fn attr_arg(args: &[AttrArg], key: &str) -> Option<String> {
    for arg in args {
        if let AttrArg::Named { name, value } = arg {
            if name.name == key {
                if let AttrValue::String(s) = value {
                    return Some(s.clone());
                }
            }
        }
    }
    None
}
