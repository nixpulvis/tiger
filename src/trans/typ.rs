use syntax::ast;
use ty::Type;
use env::{Env, Value};
use trans::Translate;

impl Translate for ast::Type {
    type Prime = Type;

    fn translate(&self,
                 tenv: &Env<Type>,
                 _venv: &Env<Value>) -> Self::Prime
    {
        match *self {
            ast::Type::Name(ref tdent) => {
                match tenv.get(tdent) {
                    Some(ty) => ty.clone(),
                    None => panic!("undefined type `{:?}`", tdent),
                }
            }

            ast::Type::Record(ref fields) => {
                let fields = fields.iter().map(|&(ref key, ref tdent)| {
                    match tenv.get(tdent) {
                        Some(ty) => (key.clone(), Box::new(ty.clone())),
                        None => panic!("undefined type `{:?}`", tdent),
                    }
                }).collect();
                Type::Record(fields)
            }

            ast::Type::Array(ref tdent) => {
                let aty = match tenv.get(tdent) {
                    Some(ty) => ty.resolve(),
                    None => panic!("undefined type `{:?}`", tdent),
                };
                Type::Array(Box::new(aty))
            }
        }
    }
}
