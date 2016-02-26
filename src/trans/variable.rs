use syntax::ast;
use ty::Type;
use env::{Env, Value};
use trans::{Translate, Translation};

impl Translate for ast::Variable {
    type Lower = Translation;

    fn translate(&self,
                 tenv: &mut Env<Type>,
                 venv: &mut Env<Value>) -> Translation
    {
        match *self {

        }
    }
}
