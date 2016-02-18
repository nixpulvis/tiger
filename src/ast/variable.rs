use super::{Symbol, Expression};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Variable {
	Simple(Symbol),
	Field(Box<Variable>, Symbol),
	Subscript(Box<Variable>, Box<Expression>),
}
