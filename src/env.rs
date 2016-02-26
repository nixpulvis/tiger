use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use syntax::ast::Symbol;
use ty::Type;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Env<T> {
    entries: HashMap<Symbol, T>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Value {
    Variable {
        ty: Type,
    },
    Function {
        args: Vec<Type>,
        ret: Type,
    },
}

impl Default for Env<Type> {
    fn default() -> Self {
        Env {
            entries: HashMap::new(),
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
