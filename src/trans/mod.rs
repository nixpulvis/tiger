use ty::Type;
use env::{Env, Value};

pub use self::translation::Translation;

pub trait Translate {
    type Prime;

    fn translate(&self,
                 tenv: &Env<Type>,
                 venv: &Env<Value>) -> Self::Prime;
}

impl<'a, T: Translate> Translate for &'a Vec<T> {
    type Prime = Vec<T::Prime>;

    fn translate(&self,
                 tenv: &Env<Type>,
                 venv: &Env<Value>) -> Self::Prime
    {
        self.iter().map(|e| e.translate(tenv, venv)).collect::<Vec<_>>()
    }
}

impl<T: Translate> Translate for Box<T> {
    type Prime = T::Prime;

    fn translate(&self,
                 tenv: &Env<Type>,
                 venv: &Env<Value>) -> Self::Prime
    {
        (**self).translate(tenv, venv)
    }
}

impl<T: Translate> Translate for Option<T> {
    type Prime = Option<T::Prime>;

    fn translate(&self,
                 tenv: &Env<Type>,
                 venv: &Env<Value>) -> Self::Prime
    {
        self.as_ref().map(|e| e.translate(tenv, venv))
    }
}

mod declaration;
mod expression;
mod translation;
mod typ;
mod variable;
