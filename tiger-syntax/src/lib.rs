extern crate unicode_xid;

pub mod ast;
pub mod tok;

pub use ast::Ast;

pub fn parse(source: &str) -> Result<Ast, String> {
    let lexer = tok::Lexer::new(source);
    match tiger::parse_Expression(source, lexer) {
        Ok(ast) => Ok(ast),
        Err(e) => Err(format!("{:?}", e)),
    }
}

mod tiger;
