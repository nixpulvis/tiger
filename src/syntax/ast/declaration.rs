use super::{Symbol, Expression, Type};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Declaration {
	Function {
		ident: Symbol,
		parameters: Vec<(Symbol, Symbol)>,
		result: Option<Symbol>,
		body: Box<Expression>,
	},
	Variable {
		ident: Symbol,
		tdent: Option<Symbol>,
		init: Box<Expression>,
	},
	Type {
		tdent: Symbol,
		ty: Box<Type>,
	}
}
