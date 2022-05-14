use crate::ast::{
  Node,
  expression::Expression,
  types::TypeRef,
};
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub enum Proposition {
  ThereIs { identifier: String, data_type: Node<TypeRef> },
  ForAll { identifier: String, data_type: Node<TypeRef> },
  Statement(Node<Expression>),
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub struct Formula {
  pub propositions: Vec<Node<Proposition>>,
}

impl Proposition {
  pub fn thereis(identifier: String, data_type: Node<TypeRef>) -> Box<Self> {
    Box::new(Self::ThereIs { identifier, data_type })
  }

  pub fn forall(identifier: String, data_type: Node<TypeRef>) -> Box<Self> {
    Box::new(Self::ForAll { identifier, data_type })
  }

  pub fn statement(expr: Node<Expression>) -> Box<Self> {
    Box::new(Self::Statement(expr))
  }
}

impl Formula {
  pub fn new(propositions: Vec<Node<Proposition>>) -> Box<Self> {
    Box::new(Self { propositions })
  }
}
