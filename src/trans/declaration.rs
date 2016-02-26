use syntax::ast;
use ty::Type;
use env::{Env, Value};
use trans::{Translate, Translation};

impl Translate for ast::Declaration {
    type Prime = (Env<Type>, Env<Value>);

    fn translate(&self,
                 tenv: &Env<Type>,
                 venv: &Env<Value>) -> Self::Prime
    {
        match *self {
            ast::Declaration::Function { ref ident, ref parameters, ref result, ref body } => {
                let tenv = tenv.clone();
                let mut venv = venv.clone();
                let args = parameters.iter().map(|&(_, ref t)| {
                    match tenv.get(t) {
                        Some(ty) => ty.clone(),
                        None => panic!("undefined type: `{}`.", t),
                    }
                }).collect();
                let ret = if let &Some(ref result) = result {
                    match tenv.get(result) {
                        Some(ty) => ty.clone(),
                        None => panic!("undefined type: `{}`.", result),
                    }
                } else {
                    Type::Unit
                };
                let fb = Value::Function { args: args, ret: ret };
                venv.insert(ident.clone(), fb);
                (tenv, venv)
            }

            ast::Declaration::Variable { ref ident, ref tdent, ref init } => {
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

            ast::Declaration::Type { ref tdent, ref ty } => {
                let mut tenv = tenv.clone();
                let venv = venv.clone();
                let ty = ty.translate(&tenv, &venv);
                let tb = Type::Name(tdent.clone(), Some(Box::new(ty)));
                tenv.insert(tdent.clone(), tb);
                (tenv, venv)
            }
        }
    }
}
