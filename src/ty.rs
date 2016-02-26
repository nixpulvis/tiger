use syntax::ast;
use ir;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Type {
    Record(Vec<(ast::Symbol, Box<Type>)>),
    Array(Box<Type>),
    Name(ast::Symbol, Option<Box<Type>>),
    Unit,
    Nil,
    Int,
    String,
    Bottom,
}

impl Type {
    // TODO: Lattice trait?
    pub fn unify(&self, other: &Type) -> Type {
        if self == other {
            self.clone()
        } else {
            panic!("mismatched types: expected {:?}, found {:?}", self, other);
        }
    }
}
