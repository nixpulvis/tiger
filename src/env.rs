use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use syntax::ast::Symbol;
use ty::{Type, ResolvedType, UnresolvedType};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Env<T> {
    entries: HashMap<Symbol, T>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Value<T: Type> {
    Variable {
        ty: T,
    },
    Function {
        args: Vec<T>,
        ret: T,
    },
}

impl Default for Env<Type> {
    fn default() -> Self {
        let mut base = HashMap::new();
        base.insert("int".into(), Type::Int);
        base.insert("string".into(), Type::String);
        Env {
            entries: base,
        }
    }
}

impl Default for Env<Value> {
    fn default() -> Self {
        Env {
            entries: HashMap::new(),
        }
    }
}

impl<T> Deref for Env<T> {
    type Target = HashMap<Symbol, T>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl<T> DerefMut for Env<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}

impl<'a> From<Env<Type>> for Env<UnresolvedType<'a>> {
    fn from(env: Env<Type>) -> Self {
        let entries = env.entries.into_iter().map(|(k, v)| {
            (k, Box::new(v).into())
        }).collect();
        Env {
            entries: entries,
        }
    }
}

impl<'a> From<Env<UnresolvedType<'a>>> for Env<Type> {
    fn from(env: Env<UnresolvedType<'a>>) -> Self {
        let entries = env.entries.into_iter().map(|(k, v)| {
            (k, v.into())
        }).collect();
        Env {
            entries: entries,
        }
    }
}
