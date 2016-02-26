use syntax::ast;
use ty::Type;
use env::{Env, Value};
use trans::{Translate, Translation};

impl Translate for ast::Expression {
    type Lower = Translation;

    fn translate(&self,
                 tenv: &mut Env<Type>,
                 venv: &mut Env<Value>) -> Translation
    {
        match *self {
            ast::Expression::Nil => {
                Translation {
                    ir: (),
                    ty: Type::Nil,
                }
            }

            ast::Expression::Break => {
                Translation {
                    ir: (),
                    ty: Type::Bottom,
                }
            }

            ast::Expression::Int(ref i) => {
                Translation {
                    ir: (),
                    ty: Type::Int,
                }
            }

            ast::Expression::String(ref s) => {
                Translation {
                    ir: (),
                    ty: Type::String,
                }
            }

            ast::Expression::Variable(ref v) => {
                // TODO: Translate and lookup variable.
                Translation {
                    ir: (),
                    ty: Type::String,
                }
            }

            ast::Expression::If { ref test, ref t, ref f } => {
                let test = test.translate(tenv, venv);
                let t = t.translate(tenv, venv);
                let f = f.as_ref().map(|e| e.translate(tenv, venv));

                let ty = match f {
                    Some(f) => t.ty.unify(&f.ty),
                    None => Type::Nil,
                };

                Translation {
                    ir: (),
                    ty: ty,
                }
            }

            ast::Expression::Sequence(ref expressions) => {
                let translations = expressions.translate(tenv, venv);
                let ty = translations.last().map_or(Type::Nil, |t| t.ty.clone());

                Translation {
                    ir: (),
                    ty: ty,
                }
            }

            ast::Expression::Call { ref ident, ref arguments } => {
                // TODO: Lookup ident.
                let translations = arguments.translate(tenv, venv);

                // TODO: Unify arguments and formals.
                Translation {
                    ir: (),
                    ty: Type::Int,
                }
            }

            ast::Expression::Operation { ref op, ref left, ref right } => {
                let left = left.translate(tenv, venv);
                let right = right.translate(tenv, venv);

                // TODO: Unify left and right with the correct op.
                Translation {
                    ir: (),
                    ty: Type::Int,
                }
            }
            ast::Expression::Record { ref fields, ref tdent } => {
                // TODO: Unify fields with field type.
                Translation {
                    ir: (),
                    ty: Type::Int,
                }
            }

            ast::Expression::Assign { ref variable, ref expression } => {
                // TODO: Unify variable with expression.
                Translation {
                    ir: (),
                    ty: Type::Unit,
                }
            }

            ast::Expression::While { ref test, ref body } => {
                // TODO: Unify test and body.
                Translation {
                    ir: (),
                    ty: Type::Unit,
                }
            }

            ast::Expression::For { ref ident, ref low, ref high, ref body } => {
                // TODO: Add ident to env.
                // TODO: Unify low, high, body.
                Translation {
                    ir: (),
                    ty: Type::Unit,
                }
            }

            ast::Expression::Let { ref declarations, ref body } => {
                // TODO: Extend new env with declarations.
                // TODO: Unify body in new env.
                Translation {
                    ir: (),
                    ty: Type::Unit,
                }
            }

            ast::Expression::Array { ref tdent, ref size, ref init } => {
                // TODO: Lookup tdent in env.
                let size = size.translate(tenv, venv);
                let init = init.translate(tenv, venv);
                // TODO: Unify size and init.
                Translation {
                    ir: (),
                    ty: Type::Unit,
                }
            }
        }
    }
}
