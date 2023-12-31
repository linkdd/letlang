use ast_core::*;

use crate::ast::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Effect<M> {
  pub params: Vec<Node<Type<M>, M>>,
  pub return_type: Node<Type<M>, M>,
}
