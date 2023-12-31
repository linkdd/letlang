use ast_core::*;

use crate::ast::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum ListItem<M> {
  Value(Node<Expression<M>, M>),
  Expansion(Node<Expression<M>, M>),
}
