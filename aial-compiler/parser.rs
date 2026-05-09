// parser.rs - AAL 语言语法分析器

use crate::ast::*;
use crate::token::{Span, Token, TokenKind};

/// 解析器
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Result<Program, Vec<String>> {
        let program = self.parse_program();
        let program = match program {
            Ok(p) => p,
            Err(_) => return Err(self.errors),
        };
        if self.errors.is_empty() {
            Ok(program)
        } else {
            Err(self.errors)
        }
    }

    // === 工具方法 ===

    fn peek(&self) -> &Token { self.tokens.get(self.pos).unwrap_or(&EOF_TOKEN) }
    fn peek2(&self) -> &Token { self.tokens.get(self.pos + 1).unwrap_or(&EOF_TOKEN) }

    fn advance(&mut self) -> &Token {
        let t = self.tokens.get(self.pos).unwrap_or(&EOF_TOKEN);
        self.pos += 1;
        t
    }

    fn check(&self, kind: fn(&TokenKind) -> bool) -> bool {
        kind(&self.peek().kind)
    }

    fn consume(&mut self, kind: fn(&TokenKind) -> bool) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: fn(&TokenKind) -> bool) -> Result<&Token, ()> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            self.error(format!("expected specific token, got {:?}", self.peek().kind));
            Err(())
        }
    }

    fn error(&mut self, msg: String) {
        let t = self.peek();
        self.errors.push(format!("[{}:{}] {}", t.span.line, t.span.col, msg));
    }

    fn skip_until(&mut self, sync: &[TokenKind]) {
        while !matches!(self.peek().kind, TokenKind::Eof) {
            if sync.contains(&self.peek().kind) {
                return;
            }
            self.advance();
        }
    }

    // === 程序解析 ===

    fn parse_program(&mut self) -> Result<Program, ()> {
        let mut items = Vec::new();
        let mut main_fn = None;
        while !self.check(|k| matches!(k, TokenKind::Eof)) {
            match self.parse_top_level() {
                Ok(TopLevelItem::FnDef(fn_def)) if fn_def.name.name == "main" => {
                    main_fn = Some(fn_def);
                }
                Ok(item) => items.push(item),
                Err(_) => self.skip_until(&[TokenKind::Fn, TokenKind::Struct, TokenKind::Enum, TokenKind::Eof]),
            }
        }
        Ok(Program { items, main_fn })
    }

    fn parse_top_level(&mut self) -> Result<TopLevelItem, ()> {
        let attrs = self.parse_attributes()?;

        match &self.peek().kind {
            TokenKind::Use => Ok(TopLevelItem::Use(self.parse_use()?)),
            TokenKind::Test => {
                self.advance();
                Ok(TopLevelItem::Test(self.parse_fn_def(vec![])?))
            }
            TokenKind::Fn => Ok(TopLevelItem::FnDef(self.parse_fn_def(attrs)?)),
            TokenKind::Type => Ok(TopLevelItem::TypeDef(self.parse_type_alias(attrs)?)),
            TokenKind::Struct => Ok(TopLevelItem::StructDef(self.parse_struct_def(attrs)?)),
            TokenKind::Enum => Ok(TopLevelItem::EnumDef(self.parse_enum_def(attrs)?)),
            TokenKind::Trait => Ok(TopLevelItem::TraitDef(self.parse_trait_def(attrs)?)),
            TokenKind::Impl => Ok(TopLevelItem::ImplBlock(self.parse_impl_block()?)),
            _ => {
                if !attrs.is_empty() {
                    self.error("attribute cannot be placed here".to_string());
                }
                self.error(format!("unrecognized top-level item: {:?}", self.peek().kind));
                Err(())
            }
        }
    }

    // === 使用 ===

    fn parse_use(&mut self) -> Result<UseStmt, ()> {
        let span = self.peek().span;
        self.expect(|k| matches!(k, TokenKind::Use))?;
        let path = self.parse_path()?;
        self.expect(|k| matches!(k, TokenKind::Semicolon))?;
        Ok(UseStmt { span, path })
    }

    fn parse_path(&mut self) -> Result<Path, ()> {
        let mut segments = Vec::new();
        segments.push(self.parse_ident()?);
        while self.consume(|k| matches!(k, TokenKind::ColonColon)) {
            segments.push(self.parse_ident()?);
        }
        Ok(Path { segments })
    }

    // === 函数 ===

    fn parse_fn_def(&mut self, attrs: Vec<Attribute>) -> Result<FnDef, ()> {
        let span = self.peek().span;
        self.expect(|k| matches!(k, TokenKind::Fn))?;
        let name = self.parse_ident()?;
        let generics = self.try_parse_generic_params();
        self.expect(|k| matches!(k, TokenKind::Lparen))?;
        let params = self.parse_params()?;
        self.expect(|k| matches!(k, TokenKind::Rparen))?;
        let return_type = if self.consume(|k| matches!(k, TokenKind::Arrow)) {
            Some(self.parse_type()?)
        } else {
            None
        };
        let body = self.parse_block()?;
        Ok(FnDef { span, attrs, name, generics, params, return_type, body })
    }

    fn parse_params(&mut self) -> Result<Vec<Param>, ()> {
        let mut params = Vec::new();
        if self.check(|k| matches!(k, TokenKind::Rparen)) {
            return Ok(params);
        }
        loop {
            params.push(self.parse_param()?);
            if !self.consume(|k| matches!(k, TokenKind::Comma)) {
                break;
            }
            if self.check(|k| matches!(k, TokenKind::Rparen)) { break; }
        }
        Ok(params)
    }

    fn parse_param(&mut self) -> Result<Param, ()> {
        let span = self.peek().span;
        let mutable = self.consume(|k| matches!(k, TokenKind::Mut));
        let name = self.parse_ident()?;
        self.expect(|k| matches!(k, TokenKind::Colon))?;
        let ty = self.parse_type()?;
        let default = if self.consume(|k| matches!(k, TokenKind::Assign)) {
            Some(self.parse_expr(0)?)
        } else {
            None
        };
        Ok(Param { span, mutable, name, ty, default })
    }

    fn try_parse_generic_params(&mut self) -> Option<Vec<Ident>> {
        if self.check(|k| matches!(k, TokenKind::Lt)) {
            self.advance();
            let mut params = Vec::new();
            loop {
                params.push(self.parse_ident().ok()?);
                if !self.consume(|k| matches!(k, TokenKind::Comma)) { break; }
            }
            self.expect(|k| matches!(k, TokenKind::Gt)).ok()?;
            Some(params)
        } else {
            None
        }
    }

    fn try_parse_generic_args(&mut self) -> Option<Vec<Type>> {
        if self.check(|k| matches!(k, TokenKind::Lt)) {
            self.advance();
            let mut args = Vec::new();
            loop {
                args.push(self.parse_type().ok()?);
                if !self.consume(|k| matches!(k, TokenKind::Comma)) { break; }
            }
            self.expect(|k| matches!(k, TokenKind::Gt)).ok()?;
            Some(args)
        } else {
            None
        }
    }

    // === 块 ===

    fn parse_block(&mut self) -> Result<Block, ()> {
        let span = self.peek().span;
        self.expect(|k| matches!(k, TokenKind::Lbrace))?;
        let mut stmts = Vec::new();
        loop {
            if self.check(|k| matches!(k, TokenKind::Rbrace) | matches!(k, TokenKind::Eof)) {
                break;
            }
            if let Ok(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                self.skip_until(&[TokenKind::Semicolon, TokenKind::Rbrace]);
                if self.check(|k| matches!(k, TokenKind::Semicolon)) {
                    self.advance();
                }
            }
        }
        self.expect(|k| matches!(k, TokenKind::Rbrace))?;
        Ok(Block { span, stmts, trailing_expr: None, parallel: false })
    }

    /// 用于表达式块的解析（可能有尾表达式）
    fn parse_block_with_tail(&mut self) -> Result<Block, ()> {
        let span = self.peek().span;
        self.expect(|k| matches!(k, TokenKind::Lbrace))?;
        let mut stmts = Vec::new();
        let mut trailing_expr = None;
        loop {
            if self.check(|k| matches!(k, TokenKind::Rbrace) | matches!(k, TokenKind::Eof)) {
                break;
            }
            // 尝试解析语句
            match self.parse_stmt() {
                Ok(Stmt::Expression(expr)) if self.check(|k| matches!(k, TokenKind::Rbrace)) => {
                    trailing_expr = Some(Box::new(expr));
                    break;
                }
                Ok(stmt) => stmts.push(stmt),
                Err(_) => {
                    self.skip_until(&[TokenKind::Semicolon, TokenKind::Rbrace]);
                    if self.check(|k| matches!(k, TokenKind::Semicolon)) {
                        self.advance();
                    }
                }
            }
        }
        self.expect(|k| matches!(k, TokenKind::Rbrace))?;
        Ok(Block { span, stmts, trailing_expr, parallel: false })
    }

    // === 语句 ===

    fn parse_stmt(&mut self) -> Result<Stmt, ()> {
        // #[parallel] { ... } — parallel-aware block
        if matches!(self.peek().kind, TokenKind::AttrStart) {
            let attrs = self.parse_attributes()?;
            let parallel = attrs.iter().any(|a| a.name.name == "parallel");
            if parallel && matches!(self.peek().kind, TokenKind::Lbrace) {
                let mut block = self.parse_block()?;
                let span = block.span;
                block.parallel = true;
                return Ok(Stmt::Expression(Expr {
                    kind: ExprKind::BlockExpr(block),
                    span,
                }));
            }
        }
        match &self.peek().kind {
            TokenKind::Let => self.parse_let_stmt(),
            TokenKind::If => self.parse_if_stmt(),
            TokenKind::Match => self.parse_match_stmt(),
            TokenKind::For => self.parse_for_stmt(),
            TokenKind::While => self.parse_while_stmt(),
            TokenKind::Loop => self.parse_loop_stmt(),
            TokenKind::Return => Ok(Stmt::Return(self.parse_return_stmt()?, self.peek().span)),
            TokenKind::Break => { self.advance(); self.expect(|k| matches!(k, TokenKind::Semicolon))?; Ok(Stmt::Break(self.peek().span)) }
            TokenKind::Continue => { self.advance(); self.expect(|k| matches!(k, TokenKind::Semicolon))?; Ok(Stmt::Continue(self.peek().span)) }
            _ => {
                let expr = self.parse_expr(0)?;
                if self.consume(|k| matches!(k, TokenKind::Assign)) {
                    // 赋值语句: expr = value;
                    let value = self.parse_expr(0)?;
                    self.expect(|k| matches!(k, TokenKind::Semicolon))?;
                    let target = self.expr_to_lvalue(expr, "赋值")?;
                    let span = target.span();
                    Ok(Stmt::Assign(AssignStmt { span, target, value }))
                } else if self.consume(|k| matches!(k, TokenKind::Semicolon)) {
                    Ok(Stmt::Expression(expr))
                } else if matches!(self.peek().kind, TokenKind::Rbrace) {
                    Ok(Stmt::Expression(expr))
                } else {
                    self.error("expression statement must be followed by semicolon".to_string());
                    Err(())
                }
            }
        }
    }

    fn parse_let_stmt(&mut self) -> Result<Stmt, ()> {
        let span = self.peek().span;
        self.advance(); // let
        let mutable = self.consume(|k| matches!(k, TokenKind::Mut));
        let name = self.parse_ident()?;
        let ty = if self.consume(|k| matches!(k, TokenKind::Colon)) {
            Some(self.parse_type()?)
        } else {
            None
        };
        self.expect(|k| matches!(k, TokenKind::Assign))?;
        let init = self.parse_expr(0)?;
        self.expect(|k| matches!(k, TokenKind::Semicolon))?;
        Ok(Stmt::Let(LetStmt { span, mutable, name, ty, init }))
    }

    fn parse_return_stmt(&mut self) -> Result<Option<Expr>, ()> {
        self.advance(); // return
        if self.check(|k| matches!(k, TokenKind::Semicolon)) {
            self.advance();
            Ok(None)
        } else {
            let expr = self.parse_expr(0)?;
            self.expect(|k| matches!(k, TokenKind::Semicolon))?;
            Ok(Some(expr))
        }
    }

    fn parse_if_stmt(&mut self) -> Result<Stmt, ()> {
        let span = self.peek().span;
        self.advance(); // if
        let cond = self.parse_expr(0)?;
        let then_block = self.parse_block()?;
        let else_branch = if self.consume(|k| matches!(k, TokenKind::Else)) {
            if self.check(|k| matches!(k, TokenKind::If)) {
                Some(Box::new(ElseBranch::If(Box::new(
                    match self.parse_if_stmt()? {
                        Stmt::If(if_stmt) => if_stmt,
                        _ => unreachable!(),
                    }
                ))))
            } else {
                Some(Box::new(ElseBranch::Block(self.parse_block()?)))
            }
        } else {
            None
        };
        Ok(Stmt::If(IfStmt { span, cond, then_block, else_branch }))
    }

    fn parse_match_stmt(&mut self) -> Result<Stmt, ()> {
        let span = self.peek().span;
        self.advance(); // match
        let scrutinee = self.parse_expr(0)?;
        self.expect(|k| matches!(k, TokenKind::Lbrace))?;
        let mut arms = Vec::new();
        while !self.check(|k| matches!(k, TokenKind::Rbrace) | matches!(k, TokenKind::Eof)) {
            arms.push(self.parse_match_arm()?);
        }
        self.expect(|k| matches!(k, TokenKind::Rbrace))?;
        Ok(Stmt::Match(MatchStmt { span, scrutinee, arms }))
    }

    fn parse_match_arm(&mut self) -> Result<MatchArm, ()> {
        let span = self.peek().span;
        let pattern = self.parse_pattern()?;
        self.expect(|k| matches!(k, TokenKind::FatArrow))?;
        let body = if self.check(|k| matches!(k, TokenKind::Lbrace)) {
            MatchBody::Block(self.parse_block()?)
        } else {
            let expr = self.parse_expr(0)?;
            self.consume(|k| matches!(k, TokenKind::Comma));
            MatchBody::Expr(expr)
        };
        Ok(MatchArm { span, pattern, body })
    }

    fn parse_for_stmt(&mut self) -> Result<Stmt, ()> {
        let span = self.peek().span;
        self.advance(); // for
        let variable = self.parse_ident()?;
        self.expect(|k| matches!(k, TokenKind::In))?;
        let iterator = self.parse_expr(0)?;
        let body = self.parse_block()?;
        Ok(Stmt::For(ForStmt { span, variable, iterator, body }))
    }

    fn parse_while_stmt(&mut self) -> Result<Stmt, ()> {
        let span = self.peek().span;
        self.advance(); // while
        let cond = self.parse_expr(0)?;
        let body = self.parse_block()?;
        Ok(Stmt::While(WhileStmt { span, cond, body }))
    }

    fn parse_loop_stmt(&mut self) -> Result<Stmt, ()> {
        let span = self.peek().span;
        self.advance(); // loop
        let body = self.parse_block()?;
        Ok(Stmt::Loop(LoopStmt { span, body }))
    }

    // === 表达式：优先爬升 ===

    fn parse_expr(&mut self, min_bp: u8) -> Result<Expr, ()> {
        let mut lhs = self.parse_prefix()?;
        loop {
            let op = self.peek().kind.clone();
            let bp = match &op {
                TokenKind::PipeGt => 1,
                TokenKind::OrOr => 2,
                TokenKind::AndAnd => 3,
                TokenKind::EqEq | TokenKind::NotEq => 4,
                TokenKind::Lt | TokenKind::Gt | TokenKind::LtEq | TokenKind::GtEq => 5,
                TokenKind::Plus | TokenKind::Minus => 6,
                TokenKind::Star | TokenKind::Slash | TokenKind::Percent => 7,
                TokenKind::Lparen => 9, // 函数/方法调用后缀
                TokenKind::Lbracket => 9, // 索引后缀
                TokenKind::Dot => 9, // 方法/字段访问后缀
                _ => 0,
            };
            if bp < min_bp { break; }
            match &op {
                TokenKind::PipeGt => {
                    self.advance();
                    let rhs = self.parse_expr(1)?;
                    lhs = Expr { kind: ExprKind::Pipe(Box::new(lhs.clone()), Box::new(rhs)), span: lhs.span };
                }
                TokenKind::OrOr | TokenKind::AndAnd => {
                    self.advance();
                    let rhs = self.parse_expr(bp + 1)?;
                    let op = match &op {
                        TokenKind::OrOr => BinOp::Or,
                        TokenKind::AndAnd => BinOp::And,
                        _ => unreachable!(),
                    };
                    lhs = Expr { kind: ExprKind::Binary(op, Box::new(lhs.clone()), Box::new(rhs)), span: lhs.span };
                }
                TokenKind::EqEq | TokenKind::NotEq | TokenKind::Lt | TokenKind::Gt | TokenKind::LtEq | TokenKind::GtEq => {
                    self.advance();
                    let rhs = self.parse_expr(bp + 1)?;
                    let op = match &op {
                        TokenKind::EqEq => BinOp::Eq, TokenKind::NotEq => BinOp::Ne,
                        TokenKind::Lt => BinOp::Lt, TokenKind::Gt => BinOp::Gt,
                        TokenKind::LtEq => BinOp::Le, TokenKind::GtEq => BinOp::Ge,
                        _ => unreachable!(),
                    };
                    lhs = Expr { kind: ExprKind::Binary(op, Box::new(lhs.clone()), Box::new(rhs)), span: lhs.span };
                }
                TokenKind::Plus | TokenKind::Minus => {
                    self.advance();
                    let rhs = self.parse_expr(bp + 1)?;
                    let op = match op { TokenKind::Plus => BinOp::Add, TokenKind::Minus => BinOp::Sub, _ => unreachable!() };
                    lhs = Expr { kind: ExprKind::Binary(op, Box::new(lhs.clone()), Box::new(rhs)), span: lhs.span };
                }
                TokenKind::Star | TokenKind::Slash | TokenKind::Percent => {
                    self.advance();
                    let rhs = self.parse_expr(bp + 1)?;
                    let op = match op { TokenKind::Star => BinOp::Mul, TokenKind::Slash => BinOp::Div, TokenKind::Percent => BinOp::Rem, _ => unreachable!() };
                    lhs = Expr { kind: ExprKind::Binary(op, Box::new(lhs.clone()), Box::new(rhs)), span: lhs.span };
                }
                // 后缀
                TokenKind::Lparen => {
                    self.advance();
                    let (args, named) = self.parse_call_args()?;
                    self.expect(|k| matches!(k, TokenKind::Rparen))?;
                    lhs = Expr { kind: ExprKind::Call(Box::new(lhs.clone()), args, named), span: lhs.span };
                }
                TokenKind::Lbracket => {
                    self.advance();
                    let index = self.parse_expr(0)?;
                    self.expect(|k| matches!(k, TokenKind::Rbracket))?;
                    lhs = Expr { kind: ExprKind::Index(Box::new(lhs.clone()), Box::new(index)), span: lhs.span };
                }
                TokenKind::Dot => {
                    self.advance();
                    let method = self.parse_ident()?;
                    let generic_args = self.try_parse_generic_args();
                    if self.consume(|k| matches!(k, TokenKind::Lparen)) {
                        let args = self.parse_args()?;
                        self.expect(|k| matches!(k, TokenKind::Rparen))?;
                        lhs = Expr { kind: ExprKind::MethodCall { receiver: Box::new(lhs.clone()), method, generic_args, args }, span: lhs.span };
                    } else {
                        // 字段访问 expr.field
                        lhs = Expr { kind: ExprKind::FieldAccess { receiver: Box::new(lhs.clone()), field: method }, span: lhs.span };
                    }
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    /// 前缀表达式
    fn parse_prefix(&mut self) -> Result<Expr, ()> {
        let kind = self.peek().kind.clone();
        let span = self.peek().span;
        match &kind {
            TokenKind::Int(n) => { self.advance(); Ok(Expr { kind: ExprKind::IntLiteral(*n), span }) }
            TokenKind::Float(f) => { self.advance(); Ok(Expr { kind: ExprKind::FloatLiteral(*f), span }) }
            TokenKind::String(s) => { let s = s.clone(); self.advance(); Ok(Expr { kind: ExprKind::StringLiteral(s), span }) }
            TokenKind::True => { self.advance(); Ok(Expr { kind: ExprKind::BoolLiteral(true), span }) }
            TokenKind::False => { self.advance(); Ok(Expr { kind: ExprKind::BoolLiteral(false), span }) }
            TokenKind::Null => { self.advance(); Ok(Expr { kind: ExprKind::NullLiteral, span }) }
            TokenKind::Ident(_) => {
                let path = self.parse_path()?;
                if path.segments.len() == 1 {
                    Ok(Expr { kind: ExprKind::Variable(path.segments[0].clone()), span })
                } else {
                    // 可能带构建式字面量或调用
                    if self.check(|k| matches!(k, TokenKind::Lbrace)) {
                        self.parse_struct_literal(path)
                    } else if self.check(|k| matches!(k, TokenKind::Lparen)) {
                        // 路径调用
                        self.advance();
                        let (args, named) = self.parse_call_args()?;
                        self.expect(|k| matches!(k, TokenKind::Rparen))?;
                        Ok(Expr { kind: ExprKind::Call(Box::new(Expr { kind: ExprKind::Path(path.clone()), span }), args, named), span })
                    } else {
                        Ok(Expr { kind: ExprKind::Path(path), span })
                    }
                }
            }
            TokenKind::SelfKw => { self.advance(); Ok(Expr { kind: ExprKind::SelfExpr, span }) }
            TokenKind::Ask => self.parse_ask_expr(),
            TokenKind::Receive => { self.advance(); Ok(Expr { kind: ExprKind::Receive, span }) }
            TokenKind::Lparen => {
                self.advance();
                let expr = self.parse_expr(0)?;
                self.expect(|k| matches!(k, TokenKind::Rparen))?;
                Ok(expr)
            }
            TokenKind::Lbrace => {
                let block = self.parse_block_with_tail()?;
                Ok(Expr { kind: ExprKind::BlockExpr(block), span })
            }
            TokenKind::If => {
                self.advance();
                let cond = self.parse_expr(0)?;
                let then_block = self.parse_block()?;
                self.expect(|k| matches!(k, TokenKind::Else))?;
                let else_expr = self.parse_expr(0)?;
                Ok(Expr { kind: ExprKind::IfExpr(Box::new(cond), then_block, Box::new(else_expr)), span })
            }
            TokenKind::Match => {
                self.advance();
                let scrutinee = self.parse_expr(0)?;
                self.expect(|k| matches!(k, TokenKind::Lbrace))?;
                let mut arms = Vec::new();
                while !self.check(|k| matches!(k, TokenKind::Rbrace) | matches!(k, TokenKind::Eof)) {
                    arms.push(self.parse_match_arm()?);
                }
                self.expect(|k| matches!(k, TokenKind::Rbrace))?;
                Ok(Expr { kind: ExprKind::MatchExpr(Box::new(scrutinee), arms), span })
            }
            TokenKind::Minus => {
                self.advance();
                let expr = self.parse_expr(7)?; // 一元减优先级高
                Ok(Expr { kind: ExprKind::Unary(UnOp::Neg, Box::new(expr)), span })
            }
            TokenKind::Not => {
                self.advance();
                let expr = self.parse_expr(7)?;
                Ok(Expr { kind: ExprKind::Unary(UnOp::Not, Box::new(expr)), span })
            }
            _ => {
                self.error(format!("unrecognized expression start: {:?}", kind));
                Err(())
            }
        }
    }

    // === ask 表达式 ===

    fn parse_ask_expr(&mut self) -> Result<Expr, ()> {
        let span = self.peek().span;
        self.advance(); // ask
        if self.consume(|k| matches!(k, TokenKind::Lparen)) {
            let options = self.parse_ask_options()?;
            self.expect(|k| matches!(k, TokenKind::Rparen))?;
            return Ok(Expr { kind: ExprKind::Ask(options), span });
        }
        if self.consume(|k| matches!(k, TokenKind::Dot)) {
            let method = self.parse_ident()?;
            self.expect(|k| matches!(k, TokenKind::Lparen))?;
            self.expect(|k| matches!(k, TokenKind::Lbracket))?;
            let mut groups = Vec::new();
            if !self.check(|k| matches!(k, TokenKind::Rbracket)) {
                groups.push(self.parse_ask_group()?);
                while self.consume(|k| matches!(k, TokenKind::Comma)) {
                    groups.push(self.parse_ask_group()?);
                }
            }
            self.expect(|k| matches!(k, TokenKind::Rbracket))?;
            self.expect(|k| matches!(k, TokenKind::Rparen))?;
            let kind = match method.name.as_str() {
                "many" => ExprKind::AskMany(groups),
                "race" => ExprKind::AskRace(groups),
                _ => { self.error("ask must be followed by many, race, or (".to_string()); return Err(()); }
            };
            return Ok(Expr { kind, span });
        }
        self.error("ask must be followed by ( or .".to_string());
        Err(())
    }

    fn parse_ask_group(&mut self) -> Result<Vec<AskOption>, ()> {
        self.expect(|k| matches!(k, TokenKind::Lparen))?;
        let options = self.parse_ask_options()?;
        self.expect(|k| matches!(k, TokenKind::Rparen))?;
        Ok(options)
    }

    fn parse_ask_options(&mut self) -> Result<Vec<AskOption>, ()> {
        let mut options = Vec::new();
        loop {
            // Support bare string/expr as implicit prompt: ask("hello") → ask(prompt = "hello")
            let (name, value) = if self.peek2().kind != TokenKind::Assign {
                let value = self.parse_expr(0)?;
                (Ident { name: "prompt".into(), span: value.span }, value)
            } else {
                let name = self.parse_ident()?;
                self.expect(|k| matches!(k, TokenKind::Assign))?;
                let value = self.parse_expr(0)?;
                (name, value)
            };
            options.push(AskOption { name, value });
            if !self.consume(|k| matches!(k, TokenKind::Comma)) {
                break;
            }
        }
        Ok(options)
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, ()> {
        let mut args = Vec::new();
        if self.check(|k| matches!(k, TokenKind::Rparen)) {
            return Ok(args);
        }
        loop {
            args.push(self.parse_expr(0)?);
            if !self.consume(|k| matches!(k, TokenKind::Comma)) { break; }
        }
        Ok(args)
    }

    /// 解析函数调用的参数列表，支持命名参数 ident = expr
    fn parse_call_args(&mut self) -> Result<(Vec<Expr>, Vec<AskOption>), ()> {
        let mut positional = Vec::new();
        let mut named = Vec::new();
        if self.check(|k| matches!(k, TokenKind::Rparen)) {
            return Ok((positional, named));
        }
        loop {
            // 检测命名参数: ident = expr
            if self.peek2().kind == TokenKind::Assign {
                let name = self.parse_ident()?;
                self.expect(|k| matches!(k, TokenKind::Assign))?;
                let value = self.parse_expr(0)?;
                named.push(AskOption { name, value });
            } else {
                positional.push(self.parse_expr(0)?);
            }
            if !self.consume(|k| matches!(k, TokenKind::Comma)) { break; }
        }
        Ok((positional, named))
    }

    // === 结构体字面量 ===

    fn parse_struct_literal(&mut self, path: Path) -> Result<Expr, ()> {
        let span = self.peek().span;
        self.expect(|k| matches!(k, TokenKind::Lbrace))?;
        let mut fields = Vec::new();
        while !self.check(|k| matches!(k, TokenKind::Rbrace) | matches!(k, TokenKind::Eof)) {
            let name = self.parse_ident()?;
            self.expect(|k| matches!(k, TokenKind::Colon))?;
            let value = self.parse_expr(0)?;
            fields.push((name, value));
            if !self.consume(|k| matches!(k, TokenKind::Comma)) { break; }
        }
        self.expect(|k| matches!(k, TokenKind::Rbrace))?;
        Ok(Expr { kind: ExprKind::StructLiteral { struct_name: path, fields }, span })
    }

    // === 模式 ===

    fn parse_pattern(&mut self) -> Result<Pattern, ()> {
        let mut pattern = self.parse_primary_pattern()?;
        // 处理 | (或模式)
        while self.consume(|k| matches!(k, TokenKind::Pipe)) {
            let next = self.parse_primary_pattern()?;
            let patterns = match &mut pattern {
                Pattern::Or(patterns) => { patterns.push(next); return Ok(pattern); }
                p => vec![p.clone(), next],
            };
            pattern = Pattern::Or(patterns);
        }
        // 处理 as
        if self.consume(|k| matches!(k, TokenKind::As)) {
            let alias = self.parse_ident()?;
            pattern = Pattern::As(Box::new(pattern), alias);
        }
        Ok(pattern)
    }

    fn parse_primary_pattern(&mut self) -> Result<Pattern, ()> {
        let t = self.peek();
        let span = t.span;
        match &t.kind {
            TokenKind::Ident(name) if name == "_" => { self.advance(); Ok(Pattern::Wildcard(Ident { name: "_".into(), span })) }
            TokenKind::Ident(_) => {
                let path = self.parse_path()?;
                if path.segments.len() == 1 {
                    // 可能是变量绑定或构造器
                    if self.check(|k| matches!(k, TokenKind::Lparen)) {
                        self.advance();
                        let mut args = Vec::new();
                        loop {
                            args.push(self.parse_pattern()?);
                            if !self.consume(|k| matches!(k, TokenKind::Comma)) { break; }
                        }
                        self.expect(|k| matches!(k, TokenKind::Rparen))?;
                        Ok(Pattern::Constructor(path, args))
                    } else {
                        // 简单变量绑定（如 x）
                        Ok(Pattern::Variable(path.segments[0].clone()))
                    }
                } else {
                    Ok(Pattern::Constructor(path, vec![]))
                }
            }
            TokenKind::Int(_) | TokenKind::Float(_) | TokenKind::String(_) | TokenKind::True | TokenKind::False => {
                let expr = self.parse_prefix()?;
                Ok(Pattern::Literal(expr))
            }
            _ => { self.error("unrecognized pattern".to_string()); Err(()) }
        }
    }

    // === 类型解析 ===

    fn parse_type(&mut self) -> Result<Type, ()> {
        self.parse_union_type()
    }

    fn parse_union_type(&mut self) -> Result<Type, ()> {
        let mut ty = self.parse_simple_type()?;
        while self.consume(|k| matches!(k, TokenKind::Pipe)) {
            let next = self.parse_simple_type()?;
            match &mut ty {
                Type::Union(types) => types.push(next),
                _ => ty = Type::Union(vec![ty, next]),
            }
        }
        Ok(ty)
    }

    fn parse_simple_type(&mut self) -> Result<Type, ()> {
        let t = self.peek();
        let _span = t.span;
        match &t.kind {
            TokenKind::Ident(name) => {
                // 基础类型
                let base = match name.as_str() {
                    "int" => Some(BaseType::Int),
                    "float" => Some(BaseType::Float),
                    "bool" => Some(BaseType::Bool),
                    "string" => Some(BaseType::String),
                    "null" => Some(BaseType::Null),
                    "int8" => Some(BaseType::Int8),
                    "int16" => Some(BaseType::Int16),
                    "int32" => Some(BaseType::Int32),
                    "int64" => Some(BaseType::Int64),
                    "uint8" => Some(BaseType::Uint8),
                    "uint16" => Some(BaseType::Uint16),
                    "uint32" => Some(BaseType::Uint32),
                    "uint64" => Some(BaseType::Uint64),
                    "float32" => Some(BaseType::Float32),
                    "float64" => Some(BaseType::Float64),
                    "dynamic" => { self.advance(); return Ok(Type::Dynamic); }
                    _ => None,
                };
                if let Some(b) = base {
                    self.advance();
                    let ty = Type::Base(b);
                    // 后缀 ?
                    if self.consume(|k| matches!(k, TokenKind::Not)) {
                        return Ok(Type::Optional(Box::new(ty)));
                    }
                    return Ok(ty);
                } else {
                    // 路径类型
                    let path = self.parse_path()?;
                    let generic_args = self.try_parse_generic_args();
                    let mut ty = Type::Path(path, generic_args);
                    if self.consume(|k| matches!(k, TokenKind::Not)) {
                        ty = Type::Optional(Box::new(ty));
                    }
                    return Ok(ty);
                }
            }
            TokenKind::Fn => {
                self.advance();
                self.expect(|k| matches!(k, TokenKind::Lparen))?;
                let mut params = Vec::new();
                if !self.check(|k| matches!(k, TokenKind::Rparen)) {
                    loop {
                        params.push(self.parse_type()?);
                        if !self.consume(|k| matches!(k, TokenKind::Comma)) { break; }
                    }
                }
                self.expect(|k| matches!(k, TokenKind::Rparen))?;
                let ret = if self.consume(|k| matches!(k, TokenKind::Arrow)) {
                    self.parse_type()?
                } else {
                    Type::Base(BaseType::Null)
                };
                Ok(Type::Fn(params, Box::new(ret)))
            }
            TokenKind::Lparen => {
                self.advance();
                let ty = self.parse_type()?;
                self.expect(|k| matches!(k, TokenKind::Rparen))?;
                Ok(ty)
            }
            _ => { self.error("unrecognized type".to_string()); Err(()) }
        }
    }

    // === 注解 ===

    fn parse_attributes(&mut self) -> Result<Vec<Attribute>, ()> {
        let mut attrs = Vec::new();
        while self.check(|k| matches!(k, TokenKind::AttrStart)) {
            attrs.push(self.parse_attribute()?);
        }
        Ok(attrs)
    }

    fn parse_attribute(&mut self) -> Result<Attribute, ()> {
        self.expect(|k| matches!(k, TokenKind::AttrStart))?;
        let name = self.parse_ident()?;
        let mut args = Vec::new();
        if self.consume(|k| matches!(k, TokenKind::Lparen)) {
            loop {
                args.push(self.parse_attr_arg()?);
                if !self.consume(|k| matches!(k, TokenKind::Comma)) { break; }
            }
            self.expect(|k| matches!(k, TokenKind::Rparen))?;
        }
        self.expect(|k| matches!(k, TokenKind::Rbracket))?;
        Ok(Attribute { name, args })
    }

    fn parse_attr_arg(&mut self) -> Result<AttrArg, ()> {
        if self.peek2().kind == TokenKind::Assign {
            let name = self.parse_ident()?;
            self.advance(); // =
            let value = self.parse_attr_value()?;
            Ok(AttrArg::Named { name, value })
        } else {
            let value = self.parse_attr_value()?;
            Ok(AttrArg::Unnamed(value))
        }
    }

    fn parse_attr_value(&mut self) -> Result<AttrValue, ()> {
        match &self.peek().kind {
            TokenKind::String(s) => { let s = s.clone(); self.advance(); Ok(AttrValue::String(s)) }
            TokenKind::Int(n) => { let n = *n as i64; self.advance(); Ok(AttrValue::Int(n)) }
            TokenKind::Float(f) => { let f = *f; self.advance(); Ok(AttrValue::Float(f)) }
            TokenKind::True => { self.advance(); Ok(AttrValue::Bool(true)) }
            TokenKind::False => { self.advance(); Ok(AttrValue::Bool(false)) }
            TokenKind::Ident(s) => { let s = s.clone(); self.advance(); Ok(AttrValue::Ident(s)) }
            TokenKind::Lbracket => {
                self.advance();
                let mut items = Vec::new();
                while !self.check(|k| matches!(k, TokenKind::Rbracket) | matches!(k, TokenKind::Eof)) {
                    items.push(self.parse_attr_value()?);
                    if !self.consume(|k| matches!(k, TokenKind::Comma)) { break; }
                }
                self.expect(|k| matches!(k, TokenKind::Rbracket))?;
                Ok(AttrValue::Array(items))
            }
            _ => { self.error("unrecognized attribute value".to_string()); Err(()) }
        }
    }

    // === 其他顶层类型 ===

    fn parse_type_alias(&mut self, attrs: Vec<Attribute>) -> Result<TypeAlias, ()> {
        let span = self.peek().span;
        self.advance(); // type
        let name = self.parse_ident()?;
        let generics = self.try_parse_generic_params();
        self.expect(|k| matches!(k, TokenKind::Assign))?;
        let ty = self.parse_type()?;
        self.expect(|k| matches!(k, TokenKind::Semicolon))?;
        Ok(TypeAlias { span, attrs, name, generics, ty })
    }

    fn parse_struct_def(&mut self, attrs: Vec<Attribute>) -> Result<StructDef, ()> {
        let span = self.peek().span;
        self.advance(); // struct
        let name = self.parse_ident()?;
        let generics = self.try_parse_generic_params();
        self.expect(|k| matches!(k, TokenKind::Lbrace))?;
        let mut fields = Vec::new();
        while !self.check(|k| matches!(k, TokenKind::Rbrace) | matches!(k, TokenKind::Eof)) {
            let field_span = self.peek().span;
            let name = self.parse_ident()?;
            self.expect(|k| matches!(k, TokenKind::Colon))?;
            let ty = self.parse_type()?;
            let default = if self.consume(|k| matches!(k, TokenKind::Assign)) {
                Some(self.parse_expr(0)?)
            } else { None };
            self.consume(|k| matches!(k, TokenKind::Comma));
            fields.push(FieldDef { span: field_span, name, ty, default });
        }
        self.expect(|k| matches!(k, TokenKind::Rbrace))?;
        Ok(StructDef { span, attrs, name, generics, fields })
    }

    fn parse_enum_def(&mut self, attrs: Vec<Attribute>) -> Result<EnumDef, ()> {
        let span = self.peek().span;
        self.advance(); // enum
        let name = self.parse_ident()?;
        let generics = self.try_parse_generic_params();
        self.expect(|k| matches!(k, TokenKind::Lbrace))?;
        let mut variants = Vec::new();
        while !self.check(|k| matches!(k, TokenKind::Rbrace) | matches!(k, TokenKind::Eof)) {
            let v_span = self.peek().span;
            let name = self.parse_ident()?;
            let payload = if self.consume(|k| matches!(k, TokenKind::Lparen)) {
                let mut types = Vec::new();
                loop {
                    types.push(self.parse_type()?);
                    if !self.consume(|k| matches!(k, TokenKind::Comma)) { break; }
                }
                self.expect(|k| matches!(k, TokenKind::Rparen))?;
                Some(types)
            } else { None };
            let discriminant = if self.consume(|k| matches!(k, TokenKind::Assign)) {
                if let TokenKind::Int(n) = self.peek().kind { self.advance(); Some(n as i64) } else { None }
            } else { None };
            self.consume(|k| matches!(k, TokenKind::Comma));
            variants.push(VariantDef { span: v_span, name, payload, discriminant });
        }
        self.expect(|k| matches!(k, TokenKind::Rbrace))?;
        Ok(EnumDef { span, attrs, name, generics, variants })
    }

    fn parse_trait_def(&mut self, attrs: Vec<Attribute>) -> Result<TraitDef, ()> {
        let span = self.peek().span;
        self.advance(); // trait
        let name = self.parse_ident()?;
        let generics = self.try_parse_generic_params();
        self.expect(|k| matches!(k, TokenKind::Lbrace))?;
        let mut methods = Vec::new();
        while !self.check(|k| matches!(k, TokenKind::Rbrace) | matches!(k, TokenKind::Eof)) {
            methods.push(self.parse_method_sig()?);
        }
        self.expect(|k| matches!(k, TokenKind::Rbrace))?;
        Ok(TraitDef { span, attrs, name, generics, methods })
    }

    fn parse_method_sig(&mut self) -> Result<MethodSig, ()> {
        let span = self.peek().span;
        self.expect(|k| matches!(k, TokenKind::Fn))?;
        let name = self.parse_ident()?;
        let generics = self.try_parse_generic_params();
        self.expect(|k| matches!(k, TokenKind::Lparen))?;
        let params = self.parse_params()?;
        self.expect(|k| matches!(k, TokenKind::Rparen))?;
        let return_type = if self.consume(|k| matches!(k, TokenKind::Arrow)) {
            Some(self.parse_type()?)
        } else { None };
        self.expect(|k| matches!(k, TokenKind::Semicolon))?;
        Ok(MethodSig { span, name, generics, params, return_type })
    }

    fn parse_impl_block(&mut self) -> Result<ImplBlock, ()> {
        let span = self.peek().span;
        self.advance(); // impl
        let generics = self.try_parse_generic_params();
        let trait_name;
        let target_type;
        if self.peek2().kind == TokenKind::For || self.peek2().kind == TokenKind::ColonColon {
            trait_name = Some(self.parse_path()?);
            self.expect(|k| matches!(k, TokenKind::For))?;
            target_type = self.parse_type()?;
        } else {
            trait_name = None;
            target_type = self.parse_type()?;
        }
        self.expect(|k| matches!(k, TokenKind::Lbrace))?;
        let mut methods = Vec::new();
        while !self.check(|k| matches!(k, TokenKind::Rbrace) | matches!(k, TokenKind::Eof)) {
            let attrs = self.parse_attributes()?;
            methods.push(self.parse_fn_def(attrs)?);
        }
        self.expect(|k| matches!(k, TokenKind::Rbrace))?;
        Ok(ImplBlock { span, generics, trait_name, target_type, methods })
    }

    // === 辅助 ===

    fn parse_ident(&mut self) -> Result<Ident, ()> {
        let kind = self.peek().kind.clone();
        let span = self.peek().span;
        self.advance();
        match &kind {
            TokenKind::Ident(name) => Ok(Ident { name: name.clone(), span }),
            _ => { self.error(format!("expected identifier, got {:?}", kind)); Err(()) }
        }
    }

    /// 将 Expr 转为 LValue（用于赋值语句左侧）
    fn expr_to_lvalue(&mut self, expr: Expr, context: &str) -> Result<LValue, ()> {
        match expr.kind {
            ExprKind::Variable(ident) => Ok(LValue::Variable(ident)),
            _ => {
                self.error(format!("{}: left side of assignment is not a valid lvalue", context));
                Err(())
            }
        }
    }
}

fn lvalue_span(lv: &LValue) -> Span {
    match lv {
        LValue::Variable(ident) => ident.span,
        LValue::Field(base, _) => lvalue_span(base),
        LValue::Index(base, _) => lvalue_span(base),
        LValue::Deref(base) => lvalue_span(base),
    }
}

impl LValue {
    fn span(&self) -> Span { lvalue_span(self) }
}

// 虚拟 EOF token
static EOF_TOKEN: Token = Token { kind: TokenKind::Eof, span: Span { start: 0, end: 0, line: 0, col: 0 } };
