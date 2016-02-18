use super::{Symbol, Variable, Declaration, Operation};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Nil,
    Break,
	Int(i32),
	String(String),
	Variable(Box<Variable>),
	If {
        test: Box<Expression>,
        t: Box<Expression>,
        f: Option<Box<Expression>>,
    },
	Sequence(Vec<Box<Expression>>),
	Call {
		ident: Symbol,
		arguments: Vec<Box<Expression>>,
	},
	Operation {
		op: Operation,
		left: Box<Expression>,
		right: Box<Expression>,
	},
	Record {
		fields: Vec<(Symbol, Box<Expression>)>,
		tdent: Symbol,
	},
	Assign {
		ident: Symbol,
		expression: Box<Expression>,
	},
	While {
		test: Box<Expression>,
		body: Box<Expression>,
	},
	For {
		ident: Symbol,
		low: Box<Expression>,
		high: Box<Expression>,
		body: Box<Expression>,
	},
	Let {
		declarations: Vec<Box<Declaration>>,
		body: Box<Expression>,
	},
	Array {
		tdent: Symbol,
		size: Box<Expression>,
		init: Box<Expression>,
	},
}
