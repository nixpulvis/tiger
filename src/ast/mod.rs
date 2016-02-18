pub type Symbol = String;

pub use self::declaration::Declaration;
pub use self::expression::Expression;
pub use self::operation::Operation;
pub use self::typ::Type;
pub use self::variable::Variable;

mod declaration;
mod expression;
mod operation;
mod typ;
mod variable;
