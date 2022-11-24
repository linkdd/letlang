use crate::{
  Node,
  expression::Expression,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Structure {
  pub members: Vec<(String, Node<Expression>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tuple {
  pub members: Vec<Node<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
  pub items: Vec<Node<Expression>>,
}


impl Expression {
  pub fn structure(members: Vec<(String, Node<Expression>)>) -> Box<Self> {
    Box::new(Self::Structure(Structure { members }))
  }

  pub fn tuple(members: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::Tuple(Tuple { members }))
  }

  pub fn list(items: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::List(List { items }))
  }
}
