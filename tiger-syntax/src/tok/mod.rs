pub use self::lexer::Lexer;

pub type Span<T> = (usize, T, usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok<'input> {
    Symbol(&'input str),
    Int(&'input str),
    String(&'input str),
    Nil,
    Break,
    If,
    Then,
    Else,
    Type,
    Function,
    Var,
    Assign,
    While,
    For,
    To,
    Do,
    Let,
    In,
    End,
    Hash,  // TODO: This is a hack for now.
    Array,
    Of,
    Colon,
    Semi,
    Comma,
    LParen,
    RParen,
    LBrack,
    RBrack,
    LBrace,
    RBrace,
    Dot,
    Eq,
    Neq,
    And,
    Or,
    Plus,
    Minus,
    Times,
    Divide,
    Lt,
    Le,
    Gt,
    Ge,
}

mod lexer;
