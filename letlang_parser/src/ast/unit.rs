use crate::ast::{Node, Statement};
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub struct Unit {
  pub identifier: String,
  pub statements: Vec<Node<Statement>>,
}

impl Unit {
  pub fn new(identifier: String, statements: Vec<Node<Statement>>) -> Box<Self> {
    Box::new(Self { identifier, statements })
  }
}
