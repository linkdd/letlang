use crate::{
  Node,
  types::TypeRef,
  expression::Expression,
};


#[derive(NodeAttributes, Debug, Clone, PartialEq)]
#[node_attrs(PropositionAttributes)]
pub enum Proposition {
  Evaluation(Node<Expression>),
  Constraint(Constraint),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropositionAttributes {
  pub scope_id: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
  pub symbol_name: String,
  pub symbol_type: Node<TypeRef>,
  pub checks: Vec<Node<Expression>>,
}

impl Proposition {
  pub fn evaluation(node: Node<Expression>) -> Box<Self> {
    Box::new(Self::Evaluation(node))
  }

  pub fn constraint(symbol_name: String, symbol_type: Node<TypeRef>, checks: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::Constraint(Constraint { symbol_name, symbol_type, checks }))
  }
}
