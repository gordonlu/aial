// symbol.rs — AAL symbol table and name resolution

use crate::ast::*;
use crate::token::Span;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Variable { mutable: bool, ty: Option<Type> },
    Parameter { index: usize, ty: Type },
    Function { generics: Vec<String>, params: Vec<(String, Type)>, return_type: Option<Type> },
    TypeAlias { generics: Vec<String>, ty: Type },
    Struct { generics: Vec<String>, fields: HashMap<String, (Type, Option<Expr>)> },
    Enum { generics: Vec<String>, variants: HashMap<String, VariantInfo> },
    Trait { generics: Vec<String>, methods: Vec<MethodSig> },
    Module,
}

#[derive(Debug, Clone)]
pub struct VariantInfo {
    pub payload: Option<Vec<Type>>,
    pub discriminant: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct SymbolEntry {
    pub kind: SymbolKind,
    pub span: Span,
    pub public: bool,
}

struct Scope {
    parent: Option<usize>,
    bindings: HashMap<String, SymbolEntry>,
}

impl Scope {
    fn new(parent: Option<usize>) -> Self {
        Scope { parent, bindings: HashMap::new() }
    }

    fn lookup(&self, name: &str) -> Option<&SymbolEntry> {
        self.bindings.get(name)
    }
}

pub struct SymbolTable {
    scopes: Vec<Scope>,
    current_scope: usize,
    global_items: HashMap<String, SymbolEntry>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = SymbolTable {
            scopes: Vec::new(),
            current_scope: 0,
            global_items: HashMap::new(),
        };
        table.scopes.push(Scope::new(None));
        table
    }

    fn push_scope(&mut self) {
        let parent = self.current_scope;
        self.scopes.push(Scope::new(Some(parent)));
        self.current_scope = self.scopes.len() - 1;
    }

    fn pop_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current_scope].parent {
            self.current_scope = parent;
        }
    }

    fn define(&mut self, name: String, entry: SymbolEntry) -> Result<(), String> {
        if self.scopes[self.current_scope].bindings.contains_key(&name) {
            Err(format!("duplicate definition `{}`", name))
        } else {
            self.scopes[self.current_scope].bindings.insert(name, entry);
            Ok(())
        }
    }

    fn define_global(&mut self, name: String, entry: SymbolEntry) -> Result<(), String> {
        if self.global_items.contains_key(&name) {
            Err(format!("duplicate global `{}`", name))
        } else {
            self.global_items.insert(name, entry);
            Ok(())
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&SymbolEntry> {
        let mut scope_idx = Some(self.current_scope);
        while let Some(idx) = scope_idx {
            if let Some(entry) = self.scopes[idx].lookup(name) {
                return Some(entry);
            }
            scope_idx = self.scopes[idx].parent;
        }
        self.global_items.get(name)
    }
}

pub struct NameResolver {
    symbols: SymbolTable,
    errors: Vec<String>,
}

impl NameResolver {
    pub fn new() -> Self {
        let mut symbols = SymbolTable::new();

        // Built-in: println / print
        let _ = symbols.define_global("println".to_string(), SymbolEntry {
            kind: SymbolKind::Function {
                generics: vec![],
                params: vec![("value".to_string(), Type::Base(BaseType::String))],
                return_type: Some(Type::Base(BaseType::Null)),
            },
            span: Span { start: 0, end: 0, line: 0, col: 0 },
            public: true,
        });
        let _ = symbols.define_global("print".to_string(), SymbolEntry {
            kind: SymbolKind::Function {
                generics: vec![],
                params: vec![("value".to_string(), Type::Base(BaseType::String))],
                return_type: Some(Type::Base(BaseType::Null)),
            },
            span: Span { start: 0, end: 0, line: 0, col: 0 },
            public: true,
        });
        let _ = symbols.define_global("ask_raw".to_string(), SymbolEntry {
            kind: SymbolKind::Function {
                generics: vec![],
                params: vec![("model".to_string(), Type::Base(BaseType::Int)), ("prompt".to_string(), Type::Base(BaseType::String)), ("max_tokens".to_string(), Type::Base(BaseType::Int))],
                return_type: Some(Type::Base(BaseType::String)),
            },
            span: Span { start: 0, end: 0, line: 0, col: 0 },
            public: true,
        });

        // Built-in: string functions
        for (name, params, ret) in &[
            ("strlen", vec![("s".to_string(), Type::Base(BaseType::String))], Type::Base(BaseType::Int)),
            ("strcat", vec![("a".to_string(), Type::Base(BaseType::String)), ("b".to_string(), Type::Base(BaseType::String))], Type::Base(BaseType::String)),
            ("strslice", vec![("s".to_string(), Type::Base(BaseType::String)), ("start".to_string(), Type::Base(BaseType::Int)), ("len".to_string(), Type::Base(BaseType::Int))], Type::Base(BaseType::String)),
            ("str_eq", vec![("a".to_string(), Type::Base(BaseType::String)), ("b".to_string(), Type::Base(BaseType::String))], Type::Base(BaseType::Bool)),
            ("starts_with", vec![("s".to_string(), Type::Base(BaseType::String)), ("prefix".to_string(), Type::Base(BaseType::String))], Type::Base(BaseType::Bool)),
            ("strchr", vec![("s".to_string(), Type::Base(BaseType::String)), ("idx".to_string(), Type::Base(BaseType::Int))], Type::Base(BaseType::Int)),
        ] {
            let _ = symbols.define_global(name.to_string(), SymbolEntry {
                kind: SymbolKind::Function { generics: vec![], params: params.clone(), return_type: Some(ret.clone()) },
                span: Span { start: 0, end: 0, line: 0, col: 0 }, public: true,
            });
        }

        // Built-in modules
        for module in &["context", "privacy", "file", "http", "json", "html", "io", "ask", "ctx", "time", "ffi", "actor"] {
            let _ = symbols.define_global(module.to_string(), SymbolEntry {
                kind: SymbolKind::Module,
                span: Span { start: 0, end: 0, line: 0, col: 0 },
                public: true,
            });
        }
        // Built-in type aliases
        for (name, ty) in &[
            ("Model", Type::Base(BaseType::Int)),
            ("Context", Type::Base(BaseType::Int)),
            ("api_key", Type::Base(BaseType::ApiKey)),
            ("Usage", Type::Dynamic),
        ] {
            let _ = symbols.define_global(name.to_string(), SymbolEntry {
                kind: SymbolKind::TypeAlias { generics: vec![], ty: ty.clone() },
                span: Span { start: 0, end: 0, line: 0, col: 0 },
                public: true,
            });
        }

        // Built-in enum: AiResponse
        let mut ai_variants = HashMap::new();
        ai_variants.insert("Success".to_string(), VariantInfo { payload: None, discriminant: Some(0) });
        ai_variants.insert("Degraded".to_string(), VariantInfo { payload: None, discriminant: Some(1) });
        ai_variants.insert("Refused".to_string(), VariantInfo { payload: None, discriminant: Some(2) });
        ai_variants.insert("Error".to_string(), VariantInfo { payload: None, discriminant: Some(3) });
        let _ = symbols.define_global("AiResponse".to_string(), SymbolEntry {
            kind: SymbolKind::Enum { generics: vec![], variants: ai_variants },
            span: Span { start: 0, end: 0, line: 0, col: 0 },
            public: true,
        });

        NameResolver { symbols, errors: Vec::new() }
    }

    pub fn resolve(mut self, program: &Program) -> Result<SymbolTable, Vec<String>> {
        let _ = self.register_top_levels(program);
        for item in &program.items {
            let _ = self.resolve_top_level(item);
        }
        if let Some(main) = &program.main_fn {
            let _ = self.resolve_fn_def(main);
        }
        if self.errors.is_empty() { Ok(self.symbols) } else { Err(self.errors) }
    }

    fn error(&mut self, span: Span, msg: String) {
        self.errors.push(format!("[{}:{}] {}", span.line, span.col, msg));
    }

    fn register_top_levels(&mut self, program: &Program) -> Result<(), ()> {
        for item in &program.items {
            match item {
                TopLevelItem::FnDef(fn_def) => {
                    let entry = SymbolEntry {
                        kind: SymbolKind::Function {
                            generics: resolve_generic_names(&fn_def.generics),
                            params: resolve_param_types(&fn_def.params),
                            return_type: fn_def.return_type.clone(),
                        },
                        span: fn_def.span,
                        public: true,
                    };
                    self.symbols.define_global(fn_def.name.name.clone(), entry)
                        .map_err(|msg| self.error(fn_def.span, msg))?;
                }
                TopLevelItem::StructDef(s) => {
                    let mut fields = HashMap::new();
                    for f in &s.fields {
                        fields.insert(f.name.name.clone(), (f.ty.clone(), f.default.clone()));
                    }
                    self.symbols.define_global(s.name.name.clone(), SymbolEntry {
                        kind: SymbolKind::Struct { generics: resolve_generic_names(&s.generics), fields },
                        span: s.span, public: true,
                    }).map_err(|msg| self.error(s.span, msg))?;
                }
                TopLevelItem::EnumDef(e) => {
                    let mut variants = HashMap::new();
                    for v in &e.variants {
                        variants.insert(v.name.name.clone(), VariantInfo {
                            payload: v.payload.clone(),
                            discriminant: v.discriminant,
                        });
                    }
                    self.symbols.define_global(e.name.name.clone(), SymbolEntry {
                        kind: SymbolKind::Enum { generics: resolve_generic_names(&e.generics), variants },
                        span: e.span, public: true,
                    }).map_err(|msg| self.error(e.span, msg))?;
                }
                TopLevelItem::TypeDef(t) => {
                    self.symbols.define_global(t.name.name.clone(), SymbolEntry {
                        kind: SymbolKind::TypeAlias { generics: resolve_generic_names(&t.generics), ty: t.ty.clone() },
                        span: t.span, public: true,
                    }).map_err(|msg| self.error(t.span, msg))?;
                }
                TopLevelItem::TraitDef(tr) => {
                    self.symbols.define_global(tr.name.name.clone(), SymbolEntry {
                        kind: SymbolKind::Trait { generics: resolve_generic_names(&tr.generics), methods: tr.methods.clone() },
                        span: tr.span, public: true,
                    }).map_err(|msg| self.error(tr.span, msg))?;
                }
                TopLevelItem::Use(u) => {
                    let last = u.path.segments.last().unwrap().name.clone();
                    self.symbols.define_global(last, SymbolEntry {
                        kind: SymbolKind::Module, span: u.span, public: true,
                    }).map_err(|msg| self.error(u.span, msg))?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn resolve_top_level(&mut self, item: &TopLevelItem) -> Result<(), ()> {
        match item {
            TopLevelItem::FnDef(fn_def) => self.resolve_fn_def(fn_def)?,
            TopLevelItem::ImplBlock(impl_block) => {
                for method in &impl_block.methods { self.resolve_fn_def(method)?; }
            }
            _ => {}
        }
        Ok(())
    }

    fn resolve_fn_def(&mut self, fn_def: &FnDef) -> Result<(), ()> {
        self.symbols.push_scope();
        if let Some(generics) = &fn_def.generics {
            for g in generics {
                self.symbols.define(g.name.clone(), SymbolEntry {
                    kind: SymbolKind::TypeAlias { generics: vec![], ty: Type::Base(BaseType::Null) },
                    span: g.span, public: false,
                }).map_err(|msg| self.error(g.span, msg))?;
            }
        }
        for (i, param) in fn_def.params.iter().enumerate() {
            self.symbols.define(param.name.name.clone(), SymbolEntry {
                kind: SymbolKind::Parameter { index: i, ty: param.ty.clone() },
                span: param.span, public: false,
            }).map_err(|msg| self.error(param.span, msg))?;
        }
        self.resolve_block(&fn_def.body)?;
        self.symbols.pop_scope();
        Ok(())
    }

    fn resolve_block(&mut self, block: &Block) -> Result<(), ()> {
        self.symbols.push_scope();
        for stmt in &block.stmts { self.resolve_stmt(stmt)?; }
        if let Some(expr) = &block.trailing_expr { self.resolve_expr(expr)?; }
        self.symbols.pop_scope();
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), ()> {
        match stmt {
            Stmt::Let(let_stmt) => {
                self.resolve_expr(&let_stmt.init)?;
                self.symbols.define(let_stmt.name.name.clone(), SymbolEntry {
                    kind: SymbolKind::Variable { mutable: let_stmt.mutable, ty: let_stmt.ty.clone() },
                    span: let_stmt.span, public: false,
                }).map_err(|msg| self.error(let_stmt.span, msg))?;
            }
            Stmt::Assign(a) => { self.resolve_lvalue(&a.target)?; self.resolve_expr(&a.value)?; }
            Stmt::Expression(e) => { self.resolve_expr(e)?; }
            Stmt::Return(opt_expr, _) => { if let Some(expr) = opt_expr { self.resolve_expr(expr)?; } }
            Stmt::If(if_stmt) => {
                self.resolve_expr(&if_stmt.cond)?;
                self.resolve_block(&if_stmt.then_block)?;
                if let Some(else_branch) = &if_stmt.else_branch {
                    match else_branch.as_ref() {
                        ElseBranch::Block(b) => self.resolve_block(b)?,
                        ElseBranch::If(if_stmt) => { let _ = self.resolve_stmt(&Stmt::If(*if_stmt.clone())); }
                    }
                }
            }
            Stmt::Match(m) => {
                self.resolve_expr(&m.scrutinee)?;
                for arm in &m.arms {
                    self.symbols.push_scope();
                    self.resolve_pattern(&arm.pattern)?;
                    match &arm.body {
                        MatchBody::Block(b) => self.resolve_block(b)?,
                        MatchBody::Expr(e) => { self.resolve_expr(e)?; }
                    }
                    self.symbols.pop_scope();
                }
            }
            Stmt::For(f) => {
                self.symbols.push_scope();
                self.resolve_expr(&f.iterator)?;
                self.symbols.define(f.variable.name.clone(), SymbolEntry {
                    kind: SymbolKind::Variable { mutable: false, ty: None },
                    span: f.variable.span, public: false,
                }).map_err(|msg| self.error(f.span, msg))?;
                self.resolve_block(&f.body)?;
                self.symbols.pop_scope();
            }
            Stmt::While(w) => { self.resolve_expr(&w.cond)?; self.resolve_block(&w.body)?; }
            Stmt::Loop(l) => { self.resolve_block(&l.body)?; }
            Stmt::Break(_) | Stmt::Continue(_) => {}
            Stmt::Defer(block) => { self.resolve_block(block)?; }
        }
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), ()> {
        match &expr.kind {
            ExprKind::Variable(ident) => {
                if self.symbols.lookup(&ident.name).is_none() {
                    self.error(ident.span, format!("undefined name `{}`", ident.name));
                }
            }
            ExprKind::Path(path) => {
                let name = &path.segments[0].name;
                if self.symbols.lookup(name).is_none() {
                    self.error(expr.span, format!("undefined name `{}`", name));
                }
            }
            ExprKind::Call(func, args, named) => {
                self.resolve_expr(func)?;
                for arg in args { self.resolve_expr(arg)?; }
                for opt in named { self.resolve_expr(&opt.value)?; }
            }
            ExprKind::MethodCall { receiver, args, .. } => {
                self.resolve_expr(receiver)?;
                for arg in args { self.resolve_expr(arg)?; }
            }
            ExprKind::Binary(_, left, right) => { self.resolve_expr(left)?; self.resolve_expr(right)?; }
            ExprKind::Unary(_, operand) => { self.resolve_expr(operand)?; }
            ExprKind::Pipe(left, right) => { self.resolve_expr(left)?; self.resolve_expr(right)?; }
            ExprKind::Index(base, index) => { self.resolve_expr(base)?; self.resolve_expr(index)?; }
            ExprKind::FieldAccess { receiver, .. } => { self.resolve_expr(receiver)?; }
            ExprKind::StructLiteral { struct_name, fields } => {
                let name = &struct_name.segments[0].name;
                if self.symbols.lookup(name).is_none() {
                    self.error(expr.span, format!("undefined struct `{}`", name));
                }
                for (_, val) in fields { self.resolve_expr(val)?; }
            }
            ExprKind::IfExpr(cond, then_block, else_expr) => {
                self.resolve_expr(cond)?; self.resolve_block(then_block)?; self.resolve_expr(else_expr)?;
            }
            ExprKind::MatchExpr(scrutinee, arms) => {
                self.resolve_expr(scrutinee)?;
                for arm in arms {
                    self.symbols.push_scope();
                    self.resolve_pattern(&arm.pattern)?;
                    match &arm.body {
                        MatchBody::Block(b) => self.resolve_block(b)?,
                        MatchBody::Expr(e) => { self.resolve_expr(e)?; }
                    }
                    self.symbols.pop_scope();
                }
            }
            ExprKind::BlockExpr(block) => { self.resolve_block(block)?; }
            ExprKind::Ask(options) => { for opt in options { self.resolve_expr(&opt.value)?; } }
            ExprKind::AskMany(groups) | ExprKind::AskRace(groups) => {
                for group in groups { for opt in group { self.resolve_expr(&opt.value)?; } }
            }
            _ => {}
        }
        Ok(())
    }

    fn resolve_lvalue(&mut self, lv: &LValue) -> Result<(), ()> {
        match lv {
            LValue::Variable(ident) => {
                if self.symbols.lookup(&ident.name).is_none() {
                    self.error(ident.span, format!("undefined name `{}`", ident.name));
                }
            }
            LValue::Field(base, _) => { self.resolve_lvalue(base)?; }
            LValue::Index(base, index) => { self.resolve_lvalue(base)?; self.resolve_expr(index)?; }
            LValue::Deref(base) => { self.resolve_lvalue(base)?; }
        }
        Ok(())
    }

    fn resolve_pattern(&mut self, pattern: &Pattern) -> Result<(), ()> {
        match pattern {
            Pattern::Variable(ident) => {
                self.symbols.define(ident.name.clone(), SymbolEntry {
                    kind: SymbolKind::Variable { mutable: false, ty: None },
                    span: ident.span, public: false,
                }).map_err(|msg| self.error(ident.span, msg))?;
            }
            Pattern::Constructor(path, sub_patterns) => {
                let name = &path.segments[0].name;
                if self.symbols.lookup(name).is_none() {
                    self.error(pattern_span(pattern), format!("undefined type `{}`", name));
                }
                for sub in sub_patterns { self.resolve_pattern(sub)?; }
            }
            Pattern::Or(patterns) => { for p in patterns { self.resolve_pattern(p)?; } }
            Pattern::As(inner, alias) => {
                self.resolve_pattern(inner)?;
                self.symbols.define(alias.name.clone(), SymbolEntry {
                    kind: SymbolKind::Variable { mutable: false, ty: None },
                    span: alias.span, public: false,
                }).map_err(|msg| self.error(alias.span, msg))?;
            }
            Pattern::Wildcard(_) | Pattern::Literal(_) => {}
        }
        Ok(())
    }
}

fn resolve_generic_names(generics: &Option<Vec<Ident>>) -> Vec<String> {
    generics.as_ref().map(|v| v.iter().map(|i| i.name.clone()).collect()).unwrap_or_default()
}

fn resolve_param_types(params: &[Param]) -> Vec<(String, Type)> {
    params.iter().map(|p| (p.name.name.clone(), p.ty.clone())).collect()
}

fn pattern_span(pattern: &Pattern) -> Span {
    match pattern {
        Pattern::Variable(i) => i.span,
        Pattern::Constructor(p, _) => p.segments[0].span,
        Pattern::Or(v) => v[0].span(),
        Pattern::As(p, _) => p.span(),
        Pattern::Wildcard(i) => i.span,
        Pattern::Literal(e) => e.span,
    }
}

impl Pattern {
    fn span(&self) -> Span { pattern_span(self) }
}
