use std::str::CharIndices;
use std::iter::Peekable;
use unicode_xid::UnicodeXID;
use super::{Tok, Span, Error};

pub struct Lexer<'input> {
    text: &'input str,
    chars: Peekable<CharIndices<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(text: &'input str, shift: usize) -> Lexer<'input> {
        Lexer {
            text: text,
            chars: text.char_indices().peekable(),
        }
    }

    /// Peek at the first character on the input. Returns it's offset and
    /// value if there is a character left in the input.
    fn peek(&mut self) -> Option<(usize, char)> {
        self.chars.peek().map(|&(o, c)| (o, c))
    }

    /// Shift one character in the input if possible.
    fn shift(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }

    fn take_while<F>(&mut self, mut cond: F) -> Option<usize>
        where F: FnMut(char) -> bool
    {
        self.take_until(|c| !cond(c))
    }

    fn take_until<F>(&mut self, mut cond: F) -> Option<usize>
        where F: FnMut(char) -> bool
    {
        loop {
            match self.peek() {
                None => return None,
                Some((idx1, c)) => {
                    if cond(c) {
                        return Some(idx1);
                    } else {
                        self.shift();
                    }
                }
            }
        }
    }

    fn identifierish(&mut self, idx0: usize) -> Span<Tok<'input>> {
        let (start, word, end) = match self.take_while(is_identifier_continue) {
            Some(end) => (idx0, &self.text[idx0..end], end),
            None => (idx0, &self.text[idx0..], self.text.len()),
        };

        // TODO: Keywords.
        (start, Tok::Symbol(word), end)
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<Span<Tok<'input>>, Error>;

    fn next(&mut self) -> Option<Result<Span<Tok<'input>>, Error>> {
        loop {
            return match self.shift() {
                Some((idx0, c)) if is_identifier_start(c) => {
                    Some(Ok(self.identifierish(idx0)))
                },
                Some((idx0, ':')) => {
                    match self.peek() {
                        Some((idx1, '=')) => {
                            self.shift();
                            Some(Ok((idx0, Tok::Assign, idx1+1)))
                        },
                        _ => {
                            Some(Ok((idx0, Tok::Colon, idx0+1)))
                        }
                    }
                },
                // TODO: Remove this token.
                Some((idx0, '#')) => {
                    Some(Ok((idx0, Tok::Hash, idx0+1)))
                },
                Some((idx0, ';')) => {
                    Some(Ok((idx0, Tok::Semi, idx0+1)))
                },
                Some((idx0, ',')) => {
                    Some(Ok((idx0, Tok::Comma, idx0+1)))
                },
                Some((idx0, '.')) => {
                    Some(Ok((idx0, Tok::Dot, idx0+1)))
                },
                Some((idx0, '(')) => {
                    Some(Ok((idx0, Tok::LParen, idx0+1)))
                },
                Some((idx0, ')')) => {
                    Some(Ok((idx0, Tok::RParen, idx0+1)))
                },
                Some((idx0, '[')) => {
                    Some(Ok((idx0, Tok::LBrack, idx0+1)))
                },
                Some((idx0, ']')) => {
                    Some(Ok((idx0, Tok::RBrack, idx0+1)))
                },
                Some((idx0, '{')) => {
                    Some(Ok((idx0, Tok::LBrace, idx0+1)))
                },
                Some((idx0, '}')) => {
                    Some(Ok((idx0, Tok::RBrace, idx0+1)))
                },
                Some((idx0, '=')) => {
                    Some(Ok((idx0, Tok::Eq, idx0+1)))
                },
                Some((idx0, '<')) => {
                    match self.peek() {
                        Some((idx1, '>')) => {
                            self.shift();
                            Some(Ok((idx0, Tok::Neq, idx1+1)))
                        },
                        Some((idx1, '=')) => {
                            self.shift();
                            Some(Ok((idx0, Tok::Le, idx1+1)))
                        },
                        _ => {
                            Some(Ok((idx0, Tok::Lt, idx0+1)))
                        }
                    }
                },
                Some((idx0, '&')) => {
                    Some(Ok((idx0, Tok::And, idx0+1)))
                },
                Some((idx0, '|')) => {
                    Some(Ok((idx0, Tok::Or, idx0+1)))
                },
                Some((idx0, '+')) => {
                    Some(Ok((idx0, Tok::Plus, idx0+1)))
                },
                Some((idx0, '-')) => {
                    Some(Ok((idx0, Tok::Minus, idx0+1)))
                },
                Some((idx0, '*')) => {
                    Some(Ok((idx0, Tok::Times, idx0+1)))
                },
                Some((idx0, '/')) => {
                    Some(Ok((idx0, Tok::Divide, idx0+1)))
                },
                Some((idx0, '>')) => {
                    match self.peek() {
                        Some((idx1, '=')) => {
                            self.shift();
                            Some(Ok((idx0, Tok::Ge, idx1+1)))
                        },
                        _ => {
                            Some(Ok((idx0, Tok::Gt, idx0+1)))
                        }
                    }
                },
                Some((_, c)) if c.is_whitespace() => {
                    self.shift();
                    continue;
                },
                Some((idx, _)) => {
                    Some(Err(Error))
                },
                None => {
                    None
                },
            }
        }
    }
}

fn is_identifier_start(c: char) -> bool {
    UnicodeXID::is_xid_start(c)
}

fn is_identifier_continue(c: char) -> bool {
    UnicodeXID::is_xid_continue(c)
}
