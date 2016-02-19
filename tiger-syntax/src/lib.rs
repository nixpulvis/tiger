pub mod ast;

pub fn compile(source: &str) -> Box<ast::Expression> {
    match tiger::parse_Expression(source) {
        Ok(ast) => ast,
        Err(e) => {
            println!("{:?}", e);
            Box::new(ast::Expression::Nil)
        }
    }
}

mod tiger;
