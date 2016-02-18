#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Operation {
	Plus,
	Minus,
	Times,
	Divide,
	Eq,
	Neq,
	Lt,
	Le,
	Gt,
	Ge,
}
