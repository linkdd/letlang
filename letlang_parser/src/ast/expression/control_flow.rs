use crate::ast::{
  Node,
  expression::Expression,
  types::TypeRef,
};

use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub struct FlowMatch {
  pub value: Node<Expression>,
  pub clauses: Vec<Node<FlowMatchClause>>,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub struct FlowMatchClause {
  pub pattern: Node<TypeRef>,
  pub body: Vec<Node<Expression>>,
}

impl FlowMatch {
  pub fn new(value: Node<Expression>, clauses: Vec<Node<FlowMatchClause>>) -> Box<Self> {
    Box::new(FlowMatch { value, clauses })
  }
}

impl FlowMatchClause {
  pub fn new(pattern: Node<TypeRef>, body: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(FlowMatchClause { pattern, body })
  }
}
