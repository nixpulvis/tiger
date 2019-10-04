#[macro_use]
extern crate lalrpop_util;
extern crate unicode_xid;

pub mod ast;
pub mod tok;

pub use ast::Ast;

pub fn parse(source: &str) -> Result<Ast, String> {
    let lexer = tok::Lexer::new(source);
    match tiger::ExpressionParser::new().parse(source, lexer) {
        Ok(ast) => Ok(ast),
        Err(e) => Err(format!("{:?}", e)),
    }
}

lalrpop_mod!(pub tiger);
