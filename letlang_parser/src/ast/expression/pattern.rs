use crate::ast::{
  Node,
  expression::Literal,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
  Symbol(String),
  Literal(Node<Literal>),
  Tuple {
    members: Vec<Node<Pattern>>,
  },
  Struct {
    members: Vec<(String, Node<Pattern>)>,
  },
  List {
    items: Vec<Node<Pattern>>,
  },
  ListHeadTail {
    head: Node<Pattern>,
    tail: Node<Pattern>,
  },
}

impl Pattern {
  pub fn symbol(sym: String) -> Box<Self> {
    Box::new(Self::Symbol(sym))
  }

  pub fn literal(node: Node<Literal>) -> Box<Self> {
    Box::new(Self::Literal(node))
  }

  pub fn tuple(members: Vec<Node<Pattern>>) -> Box<Self> {
    Box::new(Self::Tuple { members })
  }

  pub fn structure(members: Vec<(String, Node<Pattern>)>) -> Box<Self> {
    Box::new(Self::Struct { members })
  }

  pub fn list(items: Vec<Node<Pattern>>) -> Box<Self> {
    Box::new(Self::List { items })
  }

  pub fn list_head_tail(head: Node<Pattern>, tail: Node<Pattern>) -> Box<Self> {
    Box::new(Self::ListHeadTail { head, tail })
  }
}
