use super::Symbol;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Type {
	Name(Symbol),
	Record(Vec<(Symbol, Symbol)>),
	Array(Symbol),
}


#[cfg(test)]
mod tests {
    use syntax::tiger;
    use syntax::ast::Type as T;

    macro_rules! test {
        ($source:expr, $expected:expr) => {{
            let actual = tiger::parse_Type($source).expect("failed to parse");
            assert_eq!($expected, *actual);
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
