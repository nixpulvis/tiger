use ty::Type;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Translation {
    pub ir: (),
    pub ty: Type,
}
