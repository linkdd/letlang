use crate::ast::{Node, Statement};

#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
  pub path: Vec<String>,
  pub statements: Vec<Node<Statement>>,
}

impl Unit {
  pub fn new(path: Vec<String>, statements: Vec<Node<Statement>>) -> Box<Self> {
    Box::new(Self { path, statements })
  }
}
