use syntax::ast::Declaration;
use syntax::ast::Declaration::*;
use ty;
use env::{Env, Value};
use trans::{Translate, Translation};

impl Translate for Declaration {
    type Prime = (Env<ty::Type>, Env<Value>);

    fn translate(&self,
                 tenv: &Env<ty::Type>,
                 venv: &Env<Value>) -> Self::Prime
    {
        match *self {
            Function { ref ident, ref parameters, ref result, ref body } => {
                (tenv.clone(), venv.clone())
            }

            Variable { ref ident, ref tdent, ref init } => {
                let tenv = tenv.clone();
                let mut venv = venv.clone();
                let init = init.translate(&tenv, &venv);
                if let &Some(ref tdent) = tdent {
                    match tenv.get(tdent) {
                        Some(ty) => {
                            ty.unify(&init.ty);
                        },
                        None => {
                            panic!("undefined type: `{}`.", tdent);
                        },
                    }
                }
                let vb = Value::Variable { ty: init.ty };
                venv.insert(ident.clone(), vb);
                (tenv, venv)
            }

            Type { ref tdent, ref ty } => {
                let mut tenv = tenv.clone();
                let venv = venv.clone();
                let ty = ty.translate(&tenv, &venv);
                let tb = ty::Type::Name(tdent.clone(), Some(Box::new(ty)));
                tenv.insert(tdent.clone(), tb);
                (tenv, venv)
            }
        }
    }
}
