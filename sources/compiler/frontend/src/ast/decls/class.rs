use ast_core::*;

use crate::ast::{Type, Clause};

#[derive(Debug, Clone, PartialEq)]
pub struct Class<M> {
  pub cons_param: Node<Type<M>, M>,
  pub clauses: Vec<Node<Clause<M>, M>>,
}
