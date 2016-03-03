// TODO: Optimize this.
pub type Symbol = String;

pub use self::declaration::Declaration;
pub use self::expression::Expression;
pub use self::operation::Operation;
pub use self::typ::Type;
pub use self::variable::Variable;

mod declaration;
mod expression;
mod operation;
mod typ;
mod variable;


#[cfg(test)]
mod tests {
    use tiger;
    use tok::Lexer;

    macro_rules! test {
        ($source:expr, $expected:expr) => {{
            let lexer = Lexer::new($source);
            let parse = tiger::parse_Symbol($source, lexer);
            assert_eq!($expected, parse.expect("failed to parse"));
        }};
    }

    #[test]
    fn test_symbol() {
        test!("foo", "foo");
    }

    #[test]
    fn test_simple_comment() {
        test!("/* Ignore me. */ foo", "foo");
    }

    #[test]
    fn test_nested_comment() {
        test!("/* Ignore me /* and me */ please. */ foo", "foo");
    }
}
