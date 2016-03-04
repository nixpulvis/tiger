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

            ast::Expression::Int(ref _integer) => {
                Translation {
                    ir: (),
                    ty: Type::Int,
                }
            }

            ast::Expression::String(ref _string) => {
                Translation {
                    ir: (),
                    ty: Type::String,
                }
            }

            ast::Expression::Variable(ref variable) => {
                variable.translate(tenv, venv)
            }

            ast::Expression::If { ref test, ref t, ref f } => {
                let test = test.translate(tenv, venv);
                let t = t.translate(tenv, venv);
                let f = f.translate(tenv, venv);
                Type::Int.unify(&test.ty);
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
                let arguments = arguments.translate(tenv, venv);
                let ty = match venv.get(ident) {
                    Some(&Value::Function { ref args, ref ret }) => {
                        if args.len() != arguments.len() {
                            panic!("`{}` arity mismatch, expected {} arguments, given {}.",
                                ident, args.len(), arguments.len());
                        }
                        for (formal, argument) in args.iter().zip(arguments) {
                            formal.unify(&argument.ty);
                        }
                        ret.clone()
                    },
                    Some(&Value::Variable { .. }) => {
                        panic!("`{}` is not a function.", ident);
                    }
                    None => {
                        panic!("undefined function: `{}`.", ident);
                    },
                };
                Translation {
                    ir: (),
                    ty: ty.resolve(),
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
                // TODO: What type should we unify body with?
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
                // TODO: What type should we unify body with?
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
                let size = size.translate(tenv, venv);
                let init = init.translate(tenv, venv);
                let aty = match tenv.get(tdent) {
                    Some(ty) => {
                        ty.resolve()
                    },
                    None => {
                        panic!("undefined type: `{}`", tdent);
                    },
                };
                Type::Int.unify(&size.ty);
                aty.unify(&init.ty);
                let ty = Type::Array(Box::new(aty));
                Translation {
                    ir: (),
                    ty: ty,
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

#[cfg(test)]
mod tests {
    use super::*;
    use {parse, translate};
    use ty::Type;

    #[test]
    fn check_function_call_arguments_good() {
        let source = r###"
let
    function foo(x: int, y: string) : int = 1
in
    foo(1, "works");
end
        "###;
        let ast = parse(&source).unwrap();
        let translation = translate(&ast).unwrap();
        assert_eq!(Type::Int, translation.ty);
    }

    // TODO: These *shouldn't* panic once we get translate to stop panicing.
    #[test]
    #[should_panic]
    fn check_function_call_arguments_wrong_type() {
        let source = r###"
let
    function foo(x: int, y: string) : int = 1
in
    foo(1, 2);
end
        "###;
        let ast = parse(&source).unwrap();
        assert!(translate(&ast).is_err());
    }

    // TODO: These *shouldn't* panic once we get translate to stop panicing.
    #[test]
    #[should_panic]
    fn check_function_call_arguments_too_few() {
        let source = r###"
let
    function foo(x: int, y: string) : int = 1
in
    foo();
end
        "###;
        let ast = parse(&source).unwrap();
        assert!(translate(&ast).is_err());
    }

    // TODO: These *shouldn't* panic once we get translate to stop panicing.
    #[test]
    #[should_panic]
    fn check_function_call_arguments_too_many() {
        let source = r###"
let
    function foo(x: int, y: string) : int = 1
in
    foo(1, "no work", 2);
end
        "###;
        let ast = parse(&source).unwrap();
        assert!(translate(&ast).is_err());

    }
}
