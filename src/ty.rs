use syntax::ast;
use ir;

pub trait Type: Clone {
    fn resolve(self) -> ResolvedType;

    fn unify(&self, other: &Type) -> ResolvedType {
        let resolved_self = self.clone().resolve();
        let resolved_other = other.clone().resolve();
        if resolved_self == resolved_other {
            resolved_other
        } else {
            panic!("mismatched types: expected {:?}, found {:?}",
                   resolved_self, resolved_other);
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ResolvedType {
    Record(Vec<(ast::Symbol, Box<Type>)>),
    Array(Box<Type>),
    Unit,
    Nil,
    Int,
    String,
    Bottom,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum UnresolvedType<'a> {
    Name(ast::Symbol, Option<&'a Box<Type>>),
    Resolved(Box<Type>),
}

impl<'a> UnresolvedType<'a> {
    pub fn resolve(&self) -> Type {
        match *self {
            UnresolvedType::Name(ref tdent, ref ty) => {
                if let &Some(ty) = ty {
                    *ty.clone()
                } else {
                    panic!("cannot resolve type `{:?}`", ty);
                }
            },
            UnresolvedType::Resolved(t) => *t.clone(),
        }
    }
}

impl<'a> From<UnresolvedType<'a>> for Type {
    fn from(ty: UnresolvedType<'a>) -> Self {
        ty.resolve()
    }
}

impl<'a> From<Box<Type>> for UnresolvedType<'a> {
    fn from(ty: Box<Type>) -> Self {
        UnresolvedType::Resolved(ty)
    }
}
