use super::Symbol;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Type {
    Name(Symbol),
    Record(Vec<(Symbol, Symbol)>),
    Array(Symbol),
}


#[cfg(test)]
mod tests {
    use tiger;
    use tok::Lexer;
    use ast::Type as T;

    macro_rules! test {
        ($source:expr, $expected:expr) => {{
            let lexer = Lexer::new($source);
            let parse = tiger::parse_Type($source, lexer);
            assert_eq!($expected, *parse.expect("failed to parse"));
        }};
    }

    #[test]
    fn test_name() {
        test!("foo", T::Name("foo".into()));
    }

    #[test]
    fn test_record() {
        test!("{ x: int, y: int }", T::Record(vec![
            ("x".into(), "int".into()),
            ("y".into(), "int".into()),
        ]));
    }

    #[test]
    fn test_array() {
        test!("array of int", T::Array("int".into()));
    }
}
