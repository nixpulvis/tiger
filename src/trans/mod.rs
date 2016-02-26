use ty::Type;
use env::{Env, Value};

pub use self::translation::Translation;

pub trait Translate {
    type Lower;

    fn translate(&self,
                 tenv: &mut Env<Type>,
                 venv: &mut Env<Value>) -> Self::Lower;
}

impl<'a, T: Translate> Translate for &'a Vec<T> {
    type Lower = Vec<T::Lower>;

    fn translate(&self,
                 tenv: &mut Env<Type>,
                 venv: &mut Env<Value>) -> Self::Lower
    {
        self.iter().map(|e| e.translate(tenv, venv)).collect::<Vec<_>>()
    }
}

impl<T: Translate> Translate for Box<T> {
    type Lower = T::Lower;

    fn translate(&self,
                 tenv: &mut Env<Type>,
                 venv: &mut Env<Value>) -> Self::Lower
    {
        (**self).translate(tenv, venv)
    }
}

mod translation;
mod expression;
