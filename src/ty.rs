use syntax::ast;

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
        let resolved_self = self.resolve();
        let resolved_other = other.resolve();
        if resolved_self == resolved_other {
            resolved_self
        } else {
            panic!("mismatched types: expected {:?}, found {:?}",
                   resolved_self,
                   resolved_other);
        }
    }

    pub fn resolve(&self) -> Type {
        match *self {
            Type::Name(_, ref ty) => {
                if let &Some(ref ty) = ty {
                    *ty.clone()
                } else {
                    panic!("cannot resolve type `{:?}`", ty);
                }
            },
            ref t => t.clone(),
        }
    }
}
