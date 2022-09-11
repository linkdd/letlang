use crate::{
  Node,
  expression::{Expression, Symbol, Literal},
};

#[derive(NodeAttributes, Debug, Clone, PartialEq)]
#[node_attrs(PatternAttributes)]
pub enum Pattern {
  Assign(Symbol),
  Value(Node<Expression>),
  Literal(Node<Literal>),
  Tuple(TuplePattern),
  Struct(StructPattern),
  List(ListPattern),
  ListHeadTail(ListHeadTailPattern),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PatternAttributes {
  pub scope_id: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TuplePattern {
  pub members: Vec<Node<Pattern>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructPattern {
  pub members: Vec<(String, Node<Pattern>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListPattern {
  pub items: Vec<Node<Pattern>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListHeadTailPattern {
  pub head: Node<Pattern>,
  pub tail: Node<Pattern>
}

impl Pattern {
  pub fn assign(sym: String) -> Box<Self> {
    Box::new(Self::Assign(Symbol(vec![sym])))
  }

  pub fn value(expr: Node<Expression>) -> Box<Self> {
    Box::new(Self::Value(expr))
  }

  pub fn literal(node: Node<Literal>) -> Box<Self> {
    Box::new(Self::Literal(node))
  }

  pub fn tuple(members: Vec<Node<Pattern>>) -> Box<Self> {
    Box::new(Self::Tuple(TuplePattern { members }))
  }

  pub fn structure(members: Vec<(String, Node<Pattern>)>) -> Box<Self> {
    Box::new(Self::Struct(StructPattern { members }))
  }

  pub fn list(items: Vec<Node<Pattern>>) -> Box<Self> {
    Box::new(Self::List(ListPattern { items }))
  }

  pub fn list_head_tail(head: Node<Pattern>, tail: Node<Pattern>) -> Box<Self> {
    Box::new(Self::ListHeadTail(ListHeadTailPattern { head, tail }))
  }
}
