use super::{Symbol, Variable, Declaration, Operation};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Nil,
    Break,
    Int(i32),
    String(String),
    Variable(Box<Variable>),
    If {
        test: Box<Expression>,
        t: Box<Expression>,
        f: Option<Box<Expression>>,
    },
    Sequence(Vec<Box<Expression>>),
    Call {
        ident: Symbol,
        arguments: Vec<Box<Expression>>,
    },
    Operation {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Record {
        fields: Vec<(Symbol, Box<Expression>)>,
        tdent: Symbol,
    },
    Assign {
        variable: Box<Variable>,
        expression: Box<Expression>,
    },
    While {
        test: Box<Expression>,
        body: Box<Expression>,
    },
    For {
        ident: Symbol,
        low: Box<Expression>,
        high: Box<Expression>,
        body: Box<Expression>,
    },
    Let {
        declarations: Vec<Box<Declaration>>,
        body: Box<Expression>,
    },
    Array {
        tdent: Symbol,
        size: Box<Expression>,
        init: Box<Expression>,
    },
}


#[cfg(test)]
mod tests {
    use tiger;
    use tok::Lexer;
    use ast::{
        Expression as E,
        Variable as V,
        Declaration as D,
        Operation as O,
        Type as T,
    };

    macro_rules! test {
        ($source:expr, $expected:expr) => {{
            let lexer = Lexer::new($source);
            let parse = tiger::parse_Expression($source, lexer);
            assert_eq!($expected, *parse.expect("failed to parse"));
        }};
    }

    #[test]
    fn test_nil() {
        test!("nil", E::Nil);
    }

    #[test]
    fn test_break() {
        test!("break", E::Break);
    }

    #[test]
    fn test_int() {
        test!("123", E::Int(123));
    }

    #[test]
    fn test_string() {
        test!(r#""hello""#, E::String("hello".into()));
    }

    #[test]
    fn test_variable() {
        let one = Box::new(E::Int(1));
        let foo         = Box::new(V::Simple("foo".into()));
        let foo_bar     = Box::new(V::Field(foo.clone(), "bar".into()));
        let foo_sub     = Box::new(V::Subscript(foo.clone(), one.clone()));
        let foo_bar_sub = Box::new(V::Subscript(foo_bar.clone(), one.clone()));
        test!("foo",        E::Variable(foo.clone()));
        test!("foo.bar",    E::Variable(foo_bar.clone()));
        test!("foo[1]",     E::Variable(foo_sub.clone()));
        test!("foo.bar[1]", E::Variable(foo_bar_sub.clone()));
    }

    #[test]
    fn test_if() {
        let one = Box::new(E::Int(1));
        test!("if 1 then 1", E::If {
            test: one.clone(),
            t: one.clone(),
            f: None,
        });
        test!("if 1 then 1 else 1", E::If {
            test: one.clone(),
            t: one.clone(),
            f: Some(one.clone()),
        });
    }

    #[test]
    fn test_sequence() {
        let one = Box::new(E::Int(1));
        test!("()",     E::Sequence(vec![]));
        test!("(1)",    E::Sequence(vec![one.clone()]));
        test!("(1; 1)", E::Sequence(vec![one.clone(), one.clone()]));
    }

    #[test]
    fn test_call() {
        let one = Box::new(E::Int(1));
        let foo = "foo".to_string();
        test!("foo()", E::Call {
            ident: foo.clone(),
            arguments: vec![],
        });
        test!("foo(1)", E::Call {
            ident: foo.clone(),
            arguments: vec![one.clone()],
        });
        test!("foo(1, 1)", E::Call {
            ident: foo.clone(),
            arguments: vec![one.clone(), one.clone()],
        });
    }

    #[test]
    fn test_operation() {
        macro_rules! test_operation {
            ($source:expr, $op:expr) => {
                test!($source, E::Operation {
                    op: $op,
                    left: Box::new(E::Int(1)),
                    right: Box::new(E::Int(1)),
                });
            };
        }

    	test_operation!("1 + 1", O::Plus);
    	test_operation!("1 - 1", O::Minus);
    	test_operation!("1 * 1", O::Times);
    	test_operation!("1 / 1", O::Divide);
    	test_operation!("1 = 1", O::Eq);
    	test_operation!("1 <> 1", O::Neq);
    	test_operation!("1 < 1", O::Lt);
    	test_operation!("1 <= 1", O::Le);
    	test_operation!("1 > 1", O::Gt);
    	test_operation!("1 >= 1", O::Ge);
    }

    #[test]
    fn test_record() {
        let one = Box::new(E::Int(1));
        test!("posn { x=1, y=1 }", E::Record {
            fields: vec![
                ("x".into(), one.clone()),
                ("y".into(), one.clone()),
            ],
            tdent: "posn".into(),
        });
    }

    #[test]
    fn test_assign() {
        let one = Box::new(E::Int(1));
        let foo = Box::new(V::Simple("foo".into()));
        let foo_bar = Box::new(V::Field(foo.clone(), "bar".into()));
        test!("foo := 1", E::Assign {
            variable: foo.clone(),
            expression: one.clone(),
        });
        test!("foo.bar := 1", E::Assign {
            variable: foo_bar.clone(),
            expression: one.clone(),
        });
    }

    #[test]
    fn test_while() {
        let one = Box::new(E::Int(1));
        test!("while 1 do 1", E::While {
            test: one.clone(),
            body: one.clone(),
        });
    }

    #[test]
    fn test_for() {
        let one = Box::new(E::Int(1));
        test!("for foo := 1 to 1 do 1", E::For {
            ident: "foo".into(),
            low: one.clone(),
            high: one.clone(),
            body: one.clone(),
        });
    }

    #[test]
    fn test_let() {
        let one = Box::new(E::Int(1));
        let two = Box::new(E::Int(2));
        let any = Box::new(T::Record(vec![("any".into(), "int".into())]));
        let any_dec = Box::new(D::Type {
            tdent: "any".into(),
            ty: any,
        });
        let buffer_dec = Box::new(D::Variable {
            ident: "buffer".into(),
            tdent: None,
            init: one.clone(),
        });
        let body = Box::new(E::Sequence(vec![one.clone(), two.clone()]));
        test!(r###"
let
    type any = {any : int}
    var buffer := 1
in
    1;
    2
end
"###, E::Let {
            declarations: vec![any_dec, buffer_dec],
            body: body,
        });
    }

    #[test]
    fn test_array() {
        let one = Box::new(E::Int(1));
        test!("foo [1] of 1", E::Array {
            tdent: "foo".into(),
            size: one.clone(),
            init: one.clone(),
        });
    }
}
