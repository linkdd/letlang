use ast_core::*;

use crate::ast::{Identifier, Expression};

#[derive(Debug, Clone, PartialEq)]
pub enum TupleItem<M> {
  Value(Node<Expression<M>, M>),
  Expansion(Node<Expression<M>, M>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NamedTupleItem<M> {
  Pair(Node<Identifier, M>, Node<Expression<M>, M>),
  Expansion(Node<Expression<M>, M>),
}
