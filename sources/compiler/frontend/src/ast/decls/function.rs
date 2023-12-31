use nonempty::NonEmpty;
use ast_core::*;

use crate::ast::{Type, Clause};

#[derive(Debug, Clone, PartialEq)]
pub struct Function<M> {
  pub tailrec: bool,
  pub params: Vec<Node<Type<M>, M>>,
  pub return_type: Node<Type<M>, M>,
  pub clauses: NonEmpty<Node<Clause<M>, M>>,
}
