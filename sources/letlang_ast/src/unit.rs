use crate::{Node, Statement};

#[derive(NodeAttributes, Debug, Clone, PartialEq)]
#[node_attrs(UnitAttributes)]
pub struct Unit {
  pub path: Vec<String>,
  pub statements: Vec<Node<Statement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnitAttributes {
  pub scope_id: usize,
  pub unit_key: String,
  pub dependencies: Vec<String>,
}

impl Unit {
  pub fn new(path: Vec<String>, statements: Vec<Node<Statement>>) -> Box<Self> {
    Box::new(Self { path, statements })
  }
}
