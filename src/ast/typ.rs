use super::Symbol;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Type {
	Name(Symbol),
	Record(Vec<(Symbol, Symbol)>),
	Array(Symbol),
}
