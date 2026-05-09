// types.rs - AAL 类型系统核心定义

use crate::ast::*;
use std::collections::HashMap;
use std::fmt;

/// 类型环境
#[derive(Debug, Clone)]
pub struct TypeEnv {
    /// 类型变量的替换（从类型变量到具体类型）
    substitutions: HashMap<TypeVarId, Type>,
    /// 类型变量计数器
    var_counter: u32,
}

/// 类型变量 ID
pub type TypeVarId = u32;

impl TypeEnv {
    pub fn new() -> Self {
        TypeEnv {
            substitutions: HashMap::new(),
            var_counter: 0,
        }
    }

    /// 创建新的类型变量
    pub fn fresh_var(&mut self) -> Type {
        let id = self.var_counter;
        self.var_counter += 1;
        Type::Var(id)
    }

    /// 应用已知替换，获取类型的实际表示
    pub fn resolve(&self, ty: &Type) -> Type {
        match ty {
            Type::Var(id) => {
                if let Some(sub) = self.substitutions.get(id) {
                    self.resolve(sub)
                } else {
                    ty.clone()
                }
            }
            Type::Optional(inner) => Type::Optional(Box::new(self.resolve(inner))),
            Type::Union(types) => Type::Union(types.iter().map(|t| self.resolve(t)).collect()),
            Type::Fn(params, ret) => {
                Type::Fn(
                    params.iter().map(|t| self.resolve(t)).collect(),
                    Box::new(self.resolve(ret)),
                )
            }
            Type::Path(path, generics) => {
                Type::Path(
                    path.clone(),
                    generics.as_ref().map(|g| g.iter().map(|t| self.resolve(t)).collect()),
                )
            }
            _ => ty.clone(),
        }
    }

    /// 尝试将类型变量替换为具体类型（合一）
    pub fn unify(&mut self, t1: &Type, t2: &Type) -> Result<(), String> {
        let a = self.resolve(t1);
        let b = self.resolve(t2);
        match (&a, &b) {
            (Type::Var(id), other) | (other, Type::Var(id)) => {
                if other == &a { return Ok(()); }
                // 检查 occurs check
                if occurs(*id, other) {
                    return Err("recursive type".into());
                }
                self.substitutions.insert(*id, other.clone());
                Ok(())
            }
            (Type::Base(b1), Type::Base(b2)) => {
                if b1 == b2 { Ok(()) } else { Err(format!("type mismatch: {:?} vs {:?}", b1, b2)) }
            }
            (Type::Optional(t1), Type::Optional(t2)) => self.unify(t1, t2),
            (Type::Fn(p1, r1), Type::Fn(p2, r2)) => {
                if p1.len() != p2.len() {
                    return Err("function parameter count mismatch".into());
                }
                for (a, b) in p1.iter().zip(p2) {
                    self.unify(a, b)?;
                }
                self.unify(r1, r2)
            }
            (Type::Path(path1, g1), Type::Path(path2, g2)) => {
                if path1.segments[0].name != path2.segments[0].name {
                    return Err(format!("type name mismatch: {} vs {}", path1.segments[0].name, path2.segments[0].name));
                }
                if let (Some(g1), Some(g2)) = (g1, g2) {
                    if g1.len() != g2.len() { return Err("generic parameter count mismatch".into()); }
                    for (a, b) in g1.iter().zip(g2) {
                        self.unify(a, b)?;
                    }
                }
                Ok(())
            }
            (Type::Union(u1), Type::Union(u2)) => {
                // 联合类型要求每个成员都能统一
                if u1.len() != u2.len() { return Err("union type branch count mismatch".into()); }
                for (a, b) in u1.iter().zip(u2) {
                    self.unify(a, b)?;
                }
                Ok(())
            }
            _ => Err(format!("cannot unify {:?} with {:?}", a, b)),
        }
    }
}

fn occurs(var: TypeVarId, ty: &Type) -> bool {
    match ty {
        Type::Var(id) => *id == var,
        Type::Optional(inner) => occurs(var, inner),
        Type::Union(types) => types.iter().any(|t| occurs(var, t)),
        Type::Fn(params, ret) => params.iter().any(|t| occurs(var, t)) || occurs(var, ret),
        Type::Path(_, generics) => {
            if let Some(g) = generics {
                g.iter().any(|t| occurs(var, t))
            } else {
                false
            }
        }
        _ => false,
    }
}

// 格式化
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Base(b) => write!(f, "{:?}", b),
            Type::Dynamic => write!(f, "dynamic"),
            Type::Optional(inner) => write!(f, "{}?", inner),
            Type::Union(types) => {
                let s: Vec<String> = types.iter().map(|t| t.to_string()).collect();
                write!(f, "{}", s.join(" | "))
            }
            Type::Fn(_params, ret) => write!(f, "fn({}) -> {}", "一", ret),
            Type::Path(p, _) => write!(f, "{}", p.segments[0].name),
            Type::Var(id) => write!(f, "?{}", id),
            _ => write!(f, "..."),
        }
    }
}
