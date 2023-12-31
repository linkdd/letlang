use nonempty::NonEmpty;
use ast_core::*;

use crate::ast::{Identifier, Expression, Literal};

#[derive(Debug, Clone, PartialEq)]
pub struct Clause<M> {
  pub patterns: Vec<Node<Pattern<M>, M>>,
  pub guard: Option<Node<Expression<M>, M>>,
  pub body: NonEmpty<Node<Expression<M>, M>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern<M> {
  Ignore,
  Binding(Node<Identifier, M>),
  Literal(Node<Literal, M>),
  Tuple {
    items: Vec<Node<Pattern<M>, M>>,
    strict: bool,
  },
  NamedTuple {
    items: Vec<(Node<Identifier, M>, Node<Pattern<M>, M>)>,
    strict: bool,
  },
  List {
    items: Vec<Node<Pattern<M>, M>>,
    strict: bool,
  },
  ListHeadTail {
    heads: Vec<Node<Pattern<M>, M>>,
    tail: Node<Pattern<M>, M>,
  },
  Eval(Node<Expression<M>, M>),
}
