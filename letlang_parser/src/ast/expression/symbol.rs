use crate::ast::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol(pub Vec<String>);


impl Expression {
  pub fn symbol(path: Vec<String>) -> Box<Self> {
    Box::new(Self::Symbol(Symbol(path)))
  }
}
