#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Operation {
    Plus,
    Minus,
    Times,
    Divide,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
}


#[cfg(test)]
mod tests {
    use tiger;
    use tok::Lexer;
    use ast::Operation as O;

    macro_rules! test {
        ($source:expr, $expected:expr) => {{
            let lexer = Lexer::new($source, 0);
            let parse = tiger::parse_Operation($source, lexer);
            assert_eq!($expected, parse.expect("failed to parse"));
        }};
    }

    #[test]
    fn test_plus() {
        test!("+", O::Plus);
    }
    #[test]
    fn test_minus() {
        test!("-", O::Minus);
    }
    #[test]
    fn test_times() {
        test!("*", O::Times);
    }
    #[test]
    fn test_divide() {
        test!("/", O::Divide);
    }
    #[test]
    fn test_eq() {
        test!("=", O::Eq);
    }
    #[test]
    fn test_neq() {
        test!("<>", O::Neq);
    }
    #[test]
    fn test_lt() {
        test!("<", O::Lt);
    }
    #[test]
    fn test_le() {
        test!("<=", O::Le);
    }
    #[test]
    fn test_gt() {
        test!(">", O::Gt);
    }
    #[test]
    fn test_ge() {
        test!(">=", O::Ge);
    }
}
