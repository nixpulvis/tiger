extern crate unicode_xid;

pub mod ast;
pub mod tok;

pub fn parse(source: &str) -> Result<Box<ast::Expression>, String> {
    let lexer = tok::Lexer::new(source);
    match tiger::parse_Expression(source, lexer) {
        Ok(ast) => Ok(ast),
        Err(e) => Err(format!("{:?}", e)),
    }
}

mod tiger;
