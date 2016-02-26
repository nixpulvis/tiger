use syntax::ast;
use ty::Type;
use env::{Env, Value};
use trans::{Translate, Translation};

impl Translate for ast::Type {
    type Prime = Type;

    fn translate(&self,
                 tenv: &Env<Type>,
                 venv: &Env<Value>) -> Self::Prime
    {
        match *self {
            ast::Type::Name(ref tdent) => {
                match tenv.get(tdent) {
                    Some(ty) => ty.clone(),
                    None => panic!("undefined type `{:?}`", tdent),
                }
            }

            ast::Type::Record(ref fields) => {
                Type::Bottom
            }

            ast::Type::Array(ref tdent) => {
                Type::Bottom
            }
        }
    }
}
