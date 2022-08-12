use crate::ast::{
  Node,
  types::TypeRef,
  expression::Expression,
};


#[derive(Debug, Clone, PartialEq)]
pub enum Proposition {
  Evaluation(Node<Expression>),
  Constraint {
    symbol_name: String,
    symbol_type: Node<TypeRef>,
    checks: Vec<Node<Expression>>,
  }
}

impl Proposition {
  pub fn evaluation(node: Node<Expression>) -> Box<Self> {
    Box::new(Self::Evaluation(node))
  }

  pub fn constraint(symbol_name: String, symbol_type: Node<TypeRef>, checks: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::Constraint { symbol_name, symbol_type, checks })
  }
}
