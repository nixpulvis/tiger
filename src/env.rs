use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use syntax::ast::Symbol;
use ty::Type;

pub struct Env<T> {
    entries: HashMap<Symbol, T>,
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

pub enum Value {
    Variable(Type),
    Function(Vec<Type>, Type),
}
