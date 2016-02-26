use syntax::ast::Variable;
use syntax::ast::Variable::*;
use ty::Type;
use env::{Env, Value};
use trans::{Translate, Translation};

impl Translate for Variable {
    type Prime = Translation;

    fn translate(&self,
                 tenv: &Env<Type>,
                 venv: &Env<Value>) -> Self::Prime
    {
        match *self {
            Simple(ref ident) => {
                let ty = match venv.get(ident) {
                    Some(&Value::Variable { ref ty }) => {
                        ty.clone()
                    },
                    Some(&Value::Function { .. }) => {
                        panic!("`{}` is not a variable.", ident);
                    },
                    None => {
                        panic!("undefined variable: `{}`.", ident);
                    },
                };

                Translation {
                    ir: (),
                    ty: ty,
                }
            }

            Field(ref variable, ref ident) => {
                // TODO: Look up ident in venv.
                Translation {
                    ir: (),
                    ty: Type::Bottom,
                }
            }

            Subscript(ref variable, ref expression) => {
                let variable = variable.translate(tenv, venv);
                let expression = expression.translate(tenv, venv);
                let ty = match variable.ty {
                    Type::Array(ty) => {
                        Type::Int.unify(&expression.ty);
                        *ty
                    },
                    t => {
                        panic!("subscript access on type `{:?}`", t);
                    }
                };
                Translation {
                    ir: (),
                    ty: ty,
                }
            }
        }
    }
}
