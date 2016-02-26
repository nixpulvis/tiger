use ty::Type;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Translate {
    pub ir: (),
    pub ty: Type,
}
