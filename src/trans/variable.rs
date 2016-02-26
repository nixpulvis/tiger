use syntax::ast;
use ty::Type;
use env::{Env, Value};
use trans::{Translate, Translation};

impl Translate for ast::Variable {
    type Prime = Translation;

    fn translate(&self,
                 tenv: &mut Env<Type>,
                 venv: &mut Env<Value>) -> Translation
    {
        match *self {
            ast::Variable::Simple(ref ident) => {
                // TODO: Look up ident in venv.
                Translation {
                    ir: (),
                    ty: Type::Int,
                }
            }

            ast::Variable::Field(ref variable, ref ident) => {
                // TODO: Look up ident in venv.
                Translation {
                    ir: (),
                    ty: Type::Int,
                }
            }

            ast::Variable::Subscript(ref variable, ref expression) => {
                // TODO: Look up ident in venv.
                Translation {
                    ir: (),
                    ty: Type::Int,
                }
            }
        }
    }
}
