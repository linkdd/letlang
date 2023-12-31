use nonempty::NonEmpty;
use ast_core::*;

use crate::ast::{Identifier, Literal};

#[derive(Debug, Clone, PartialEq)]
pub enum Type<M> {
  Reference {
    name: NonEmpty<Node<Identifier, M>>,
    type_params: Vec<Node<Type<M>, M>>,
  },
  Literal(Node<Literal, M>),
  Tuple(Vec<Node<Type<M>, M>>),
  NamedTuple(Vec<(Node<Identifier, M>, Node<Type<M>, M>)>),
  OneOf {
    lhs: Node<Type<M>, M>,
    rhs: Node<Type<M>, M>,
  },
  AllOf {
    lhs: Node<Type<M>, M>,
    rhs: Node<Type<M>, M>,
  },
  Not(Node<Type<M>, M>),
}
