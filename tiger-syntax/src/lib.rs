extern crate unicode_xid;

pub mod ast;
pub mod tok;

pub fn compile(source: &str) -> Box<ast::Expression> {
    let lexer = tok::Lexer::new(source);
    match tiger::parse_Expression(source, lexer) {
        Ok(ast) => ast,
        Err(e) => {
            println!("{:?}", e);
            Box::new(ast::Expression::Nil)
        }
    }
}

mod tiger;
