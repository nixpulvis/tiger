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
    use syntax::tiger;

    macro_rules! test {
        ($source:expr, $expected:expr) => {{
            let actual = tiger::parse_Symbol($source).expect("failed to parse");
            assert_eq!($expected, actual);
        }};
    }

    #[test]
    fn test_symbol() {
        test!("foo", "foo");
    }
}
