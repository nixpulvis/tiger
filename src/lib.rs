extern crate tiger_syntax as syntax;

use env::Env;
use trans::{Translate, Translation};

pub mod env;
pub mod ir;
pub mod ty;
pub mod trans;

#[macro_export]
macro_rules! error {
    ($fmt:expr, $kind:expr) => {{
        print!("{} error: ", $kind);
        print!(concat!($fmt, "\n"));
        ::std::process::exit(1);
    }};
    ($fmt:expr, $kind:expr, $($arg:tt)*) => {{
        print!("{} error: ", $kind);
        print!(concat!($fmt, "\n"), $($arg)*);
        ::std::process::exit(1);
    }};
}

pub use syntax::parse;

pub fn translate(ast: &Box<syntax::ast::Expression>) -> Result<Translation, String> {
    let tenv = Env::default();
    let venv = Env::default();
    Ok(ast.translate(&tenv, &venv))
}

pub mod driver {
    use syntax;
    use translate;

    pub fn compile(source: &str) -> Result<(), String> {
        let ast = match syntax::parse(source) {
            Ok(a) => a,
            Err(e) => {
                error!("{:?}", "syntax", e);
            }
        };
        println!("AST:");
        println!("{:#?}", ast);

        let translation = match translate(&ast) {
            Ok(t) => t,
            Err(e) => {
                error!("{:?}", "translation", e);
            },
        };
        println!("TRANSLATION");
        println!("{:?}", translation);

        Ok(())
    }
}
