use syntax::ast;
use ir;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Type {
    Record(Vec<(ast::Symbol, Box<Type>)>),
    Array(Box<Type>),
    Name(ast::Symbol, Option<Box<Type>>),
    Unit,
    Nil,
    Int,
    String,
    Bottom,
}

pub fn translate(expression: &ast::Expression) -> ir::Translate {
    match *expression {
        ast::Expression::Nil => {
            translate_nil()
        },
        ast::Expression::Break => {
            translate_break()
        },
        ast::Expression::Int(ref i) => {
            translate_int(i)
        },
        ast::Expression::String(ref s) => {
            translate_string(s)
        },
        ast::Expression::Variable(ref v) => {
            translate_variable(v)
        },
        ast::Expression::If { ref test, ref t, ref f } => {
            translate_if(test, t, f)
        },
        ast::Expression::Sequence(ref es) => {
            translate_sequence(es)
        },
        ast::Expression::Call { ref ident, ref arguments } => {
            translate_call(ident, arguments)
        },
        ast::Expression::Operation { ref op, ref left, ref right } => {
            translate_op(op, left, right)
        }
        ast::Expression::Record { ref fields, ref tdent } => {
            translate_record(fields, tdent)
        },
        ast::Expression::Assign { ref variable, ref expression } => {
            translate_assign(variable, expression)
        },
        ast::Expression::While { ref test, ref body } => {
            translate_while(test, body)
        },
        ast::Expression::For { ref ident, ref low, ref high, ref body } => {
            translate_for(ident, low, high, body)
        },
        ast::Expression::Let { ref declarations, ref body } => {
            translate_let(declarations, body)
        },
        ast::Expression::Array { ref tdent, ref size, ref init } => {
            translate_array(tdent, size, init)
        },
    }
}

fn translate_vec(vec: &Vec<Box<ast::Expression>>) -> Vec<ir::Translate> {
    vec.iter().map(|e| translate(&**e)).collect::<Vec<_>>()
}

fn translate_nil() -> ir::Translate {
    ir::Translate {
        ir: (),
        ty: Type::Nil,
    }
}

fn translate_break() -> ir::Translate {
    ir::Translate {
        ir: (),
        ty: Type::Bottom,
    }
}

fn translate_int(_i: &i32) -> ir::Translate {
    ir::Translate {
        ir: (),
        ty: Type::Int,
    }
}

fn translate_string(_s: &String) -> ir::Translate {
    ir::Translate {
        ir: (),
        ty: Type::String,
    }
}

fn translate_variable(v: &ast::Variable) -> ir::Translate {
    // TODO: Translate and lookup variable.
    ir::Translate {
        ir: (),
        ty: Type::String,
    }
}

fn translate_if(test: &ast::Expression,
                t: &ast::Expression,
                f: &Option<Box<ast::Expression>>) -> ir::Translate
{
    let test = translate(test);
    let t = translate(t);
    let f = f.as_ref().map(|e| translate(&**e));

    // TODO: Unify.
    ir::Translate {
        ir: (),
        ty: Type::Nil,
    }
}

fn translate_sequence(expressions: &Vec<Box<ast::Expression>>) -> ir::Translate
{
    let translations = translate_vec(expressions);
    let ty = translations.last().map_or(Type::Nil, |t| t.ty.clone());

    ir::Translate {
        ir: (),
        ty: ty,
    }
}

fn translate_call(ident: &ast::Symbol,
                  arguments: &Vec<Box<ast::Expression>>) -> ir::Translate
{
    // TODO: Lookup ident.
    let translations = translate_vec(arguments);

    // TODO: Unify arguments and formals.
    ir::Translate {
        ir: (),
        ty: Type::Int,
    }
}

fn translate_op(op: &ast::Operation,
                left: &ast::Expression,
                right: &ast::Expression) -> ir::Translate
{
    let left = translate(left);
    let right = translate(right);

    // TODO: Unify left and right with the correct op.
    ir::Translate {
        ir: (),
        ty: Type::Int,
    }
}

fn translate_record(fields: &Vec<(ast::Symbol, Box<ast::Expression>)>,
                    tdent: &ast::Symbol) -> ir::Translate
{
    // TODO: Unify fields with field type.
    ir::Translate {
        ir: (),
        ty: Type::Int,
    }
}

fn translate_assign(variable: &ast::Variable,
                    expression: &ast::Expression) -> ir::Translate
{
    // TODO: Unify variable with expression.
    ir::Translate {
        ir: (),
        ty: Type::Unit,
    }
}

fn translate_while(test: &ast::Expression,
                  body: &ast::Expression) -> ir::Translate
{
    // TODO: Unify test and body.
    ir::Translate {
        ir: (),
        ty: Type::Unit,
    }
}


fn translate_for(ident: &ast::Symbol,
                 low: &ast::Expression,
                 high: &ast::Expression,
                 body: &ast::Expression) -> ir::Translate
{
    // TODO: Add ident to env.
    // TODO: Unify low, high, body.
    ir::Translate {
        ir: (),
        ty: Type::Unit,
    }
}

fn translate_let(declarations: &Vec<Box<ast::Declaration>>,
                 body: &ast::Expression) -> ir::Translate
{
    // TODO: Extend new env with declarations.
    // TODO: Unify body in new env.
    ir::Translate {
        ir: (),
        ty: Type::Unit,
    }
}

fn translate_array(tdent: &ast::Symbol,
                   size: &ast::Expression,
                   init: &ast::Expression) -> ir::Translate
{
    // TODO: Lookup tdent in env.
    let size = translate(size);
    let init = translate(init);
    // TODO: Unify size and init.
    ir::Translate {
        ir: (),
        ty: Type::Unit,
    }
}
