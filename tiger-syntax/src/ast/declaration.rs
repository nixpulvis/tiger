use super::{Symbol, Expression, Type};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Declaration {
    Function {
        ident: Symbol,
        parameters: Vec<(Symbol, Symbol)>,
        result: Option<Symbol>,
        body: Box<Expression>,
    },
    Variable {
        ident: Symbol,
        tdent: Option<Symbol>,
        init: Box<Expression>,
    },
    Type {
        tdent: Symbol,
        ty: Box<Type>,
    }
}


#[cfg(test)]
mod tests {
    use tiger;
    use tok::Lexer;
    use ast::Declaration as D;
    use ast::Expression as E;
    use ast::Type as T;

    macro_rules! test {
        ($source:expr, $expected:expr) => {{
            let lexer = Lexer::new($source);
            let parse = tiger::parse_Declaration($source, lexer);
            assert_eq!($expected, *parse.expect("failed to parse"));
        }};
    }

    #[test]
    fn test_function() {
        let expected = D::Function {
            ident: "foo".into(),
            parameters: vec![("x".into(), "int".into())],
            result: None,
            body: Box::new(E::Int(1)),
        };
        test!("function foo(x: int) = 1", expected);
        let expected = D::Function {
            ident: "foo".into(),
            parameters: vec![("x".into(), "int".into())],
            result: Some("string".into()),
            body: Box::new(E::Int(1)),
        };
        test!("function foo(x: int) : string = 1", expected);
    }

    #[test]
    fn test_variable() {
        let expected = D::Variable {
            ident: "x".into(),
            tdent: None,
            init: Box::new(E::Int(1)),
        };
        test!("var x := 1", expected);
        let expected = D::Variable {
            ident: "x".into(),
            tdent: Some("a".into()),
            init: Box::new(E::Int(1)),
        };
        test!("var x : a := 1", expected);
    }

    #[test]
    fn test_type() {
        let expected = D::Type {
            tdent: "foo".into(),
            ty: Box::new(T::Name("int".into())),
        };
        test!("type foo = int", expected);
        let expected = D::Type {
            tdent: "foo".into(),
            ty: Box::new(T::Array("int".into())),
        };
        test!("type foo = array of int", expected);
        let expected = D::Type {
            tdent: "foo".into(),
            ty: Box::new(T::Record(vec![("a".into(), "int".into())])),
        };
        test!("type foo = {a: int}", expected);
    }
}
