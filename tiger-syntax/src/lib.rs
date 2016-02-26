extern crate unicode_xid;

pub mod ast;
pub mod tok;

pub fn parse(source: &str) -> Result<Box<ast::Expression>, ()> {
    let lexer = tok::Lexer::new(source);
    Ok(match tiger::parse_Expression(source, lexer) {
        Ok(ast) => ast,
        Err(e) => panic!("parse failed: {:?}", e),
    })
}

mod tiger;
