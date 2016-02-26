use syntax::ast;
use ty::Type;
use env::{Env, Value};
use trans::{Translate, Translation};

impl Translate for ast::Expression {
    type Prime = Translation;

    fn translate(&self,
                 tenv: &Env<Type>,
                 venv: &Env<Value>) -> Self::Prime
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
                v.translate(tenv, venv)
            }

            ast::Expression::If { ref test, ref t, ref f } => {
                let test = test.translate(tenv, venv);
                let t = t.translate(tenv, venv);
                let f = f.translate(tenv, venv);
                let ty = match f {
                    Some(f) => t.ty.unify(&f.ty),
                    None => Type::Unit,
                };
                Translation {
                    ir: (),
                    ty: ty,
                }
            }

            ast::Expression::Sequence(ref expressions) => {
                let expressions = expressions.translate(tenv, venv);
                let ty = expressions.last().map_or(Type::Unit, |t| t.ty.clone());
                Translation {
                    ir: (),
                    ty: ty,
                }
            }

            ast::Expression::Call { ref ident, ref arguments } => {
                // TODO: Lookup ident.
                let arguments = arguments.translate(tenv, venv);
                // TODO: Unify arguments and formals.
                Translation {
                    ir: (),
                    ty: Type::Bottom,
                }
            }

            ast::Expression::Operation { ref op, ref left, ref right } => {
                let left = left.translate(tenv, venv);
                let right = right.translate(tenv, venv);

                match *op {
                    ast::Operation::Plus => check_arith_op(&left.ty, &right.ty),
                    ast::Operation::Minus => check_arith_op(&left.ty, &right.ty),
                    ast::Operation::Times => check_arith_op(&left.ty, &right.ty),
                    ast::Operation::Divide => check_arith_op(&left.ty, &right.ty),
                    ast::Operation::Lt => check_cmp_op(&left.ty, &right.ty),
                    ast::Operation::Le => check_cmp_op(&left.ty, &right.ty),
                    ast::Operation::Gt => check_cmp_op(&left.ty, &right.ty),
                    ast::Operation::Ge => check_cmp_op(&left.ty, &right.ty),
                    ast::Operation::Eq => check_eq_op(&left.ty, &right.ty),
                    ast::Operation::Neq => check_eq_op(&left.ty, &right.ty),
                };

                Translation {
                    ir: (),
                    ty: Type::Int,
                }
            }

            ast::Expression::Record { ref tdent, ref fields } => {
                // TODO: Unify fields with field type.
                Translation {
                    ir: (),
                    ty: Type::Bottom,
                }
            }

            ast::Expression::Assign { ref variable, ref expression } => {
                let variable = variable.translate(tenv, venv);
                let expression = expression.translate(tenv, venv);
                variable.ty.unify(&expression.ty);
                Translation {
                    ir: (),
                    ty: Type::Unit,
                }
            }

            ast::Expression::While { ref test, ref body } => {
                let test = test.translate(tenv, venv);
                let body = body.translate(tenv, venv);
                Type::Int.unify(&test.ty);
                Translation {
                    ir: (),
                    ty: Type::Unit,
                }
            }

            ast::Expression::For { ref ident, ref low, ref high, ref body } => {
                // TODO: Add ident to env.
                // FIXME: What envs do we use here?
                let low = low.translate(tenv, venv);
                let high = high.translate(tenv, venv);
                let body = body.translate(tenv, venv);
                Type::Int.unify(&low.ty);
                Type::Int.unify(&high.ty);
                Translation {
                    ir: (),
                    ty: Type::Unit,
                }
            }

            ast::Expression::Let { ref declarations, ref body } => {
                let tenv = tenv.clone();
                let venv = venv.clone();
                let (tenv, venv) = declarations.iter().fold((tenv, venv), |(t, e), d| {
                    d.translate(&t, &e)
                });
                let body = body.translate(&tenv, &venv);
                Translation {
                    ir: (),
                    ty: body.ty,
                }
            }

            ast::Expression::Array { ref tdent, ref size, ref init } => {
                // TODO: Lookup tdent in env.
                let size = size.translate(tenv, venv);
                let init = init.translate(tenv, venv);
                Type::Int.unify(&size.ty);
                // TODO: Unify init with type of tdent.
                Translation {
                    ir: (),
                    ty: Type::Bottom,
                }
            }
        }
    }
}

fn check_arith_op(left: &Type, right: &Type) {
    left.unify(right);
    match *left {
        Type::Int => {},
        _ => panic!("cannot perform arithmetic on type `{:?}`", left),
    }
}

fn check_cmp_op(left: &Type, right: &Type) {
    left.unify(right);
    match *left {
        Type::Int
      | Type::String => {},
        _ => panic!("cannot compare type `{:?}`", left),
    }
}

fn check_eq_op(left: &Type, right: &Type) {
    left.unify(right);
    match *left {
        Type::Int
      | Type::String
      | Type::Record(_)
      | Type::Array(_) => {},
        _ => panic!("cannot test equality of type `{:?}`", left),
    }
}
