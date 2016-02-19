use super::{Symbol, Expression};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Variable {
	Simple(Symbol),
	Field(Box<Variable>, Symbol),
	Subscript(Box<Variable>, Box<Expression>),
}


#[cfg(test)]
mod tests {
    use syntax::tiger;
    use syntax::ast::{
        Expression as E,
        Variable as V,
    };

    macro_rules! test {
        ($source:expr, $expected:expr) => {{
            let actual = tiger::parse_Variable($source).expect("failed to parse");
            assert_eq!($expected, *actual);
        }};
    }

    #[test]
    fn test_simple() {
        let one = Box::new(E::Int(1));
        test!("foo", V::Simple("foo".into()));
    }

    #[test]
    fn test_field() {
        let one = Box::new(E::Int(1));
        let foo = Box::new(V::Simple("foo".into()));
        test!("foo.bar", V::Field(foo.clone(), "bar".into()));
    }

    #[test]
    fn test_subscript() {
        let one = Box::new(E::Int(1));
        let foo = Box::new(V::Simple("foo".into()));
        test!("foo[1]", V::Subscript(foo.clone(), one.clone()));
    }

    #[test]
    fn test_chain() {
        let one = Box::new(E::Int(1));
        let foo = Box::new(V::Simple("foo".into()));
        let foo_bar = Box::new(V::Field(foo.clone(), "bar".into()));
        let foo_sub = Box::new(V::Subscript(foo.clone(), one.clone()));
        test!("foo.bar[1]", V::Subscript(foo_bar.clone(), one.clone()));
    }
}