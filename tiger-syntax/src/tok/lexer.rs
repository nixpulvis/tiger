use std::str::CharIndices;
use super::{Tok, Span, Error};

pub struct Lexer<'input> {
    text: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
    shift: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(text: &'input str, shift: usize) -> Lexer<'input> {
        Lexer {
            text: text,
            chars: text.char_indices(),
            lookahead: None,
            shift: shift,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<Span<Tok<'input>>, Error>;

    fn next(&mut self) -> Option<Result<Span<Tok<'input>>, Error>> {
        None
    }
}
