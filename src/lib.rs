extern crate tiger_syntax as syntax;

use env::Env;
use trans::Translate;

pub mod env;
pub mod ir;
pub mod ty;
pub mod trans;

pub fn compile(source: &str) -> trans::Translation {
    let tenv = Env::default();
    let venv = Env::default();
    let ast = syntax::parse(&source).unwrap();
    ast.translate(&tenv, &venv)
}
