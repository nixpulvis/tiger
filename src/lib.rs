extern crate tiger_syntax as syntax;

use env::Env;
use trans::{Translate, Translation};

pub mod env;
pub mod ir;
pub mod ty;
pub mod trans;

pub fn compile(source: &str) -> () {
    unimplemented!();
}

pub fn translate(ast: &Box<syntax::ast::Expression>) -> Result<Translation, ()> {
    let tenv = Env::default();
    let venv = Env::default();
    Ok(ast.translate(&tenv, &venv))
}
