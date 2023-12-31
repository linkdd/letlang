use ast_core::*;

use crate::ast::Identifier;

mod class;
mod effect;
mod function;

pub use self::{
  class::*,
  effect::*,
  function::*,
};

#[derive(Debug, Clone, PartialEq)]
pub struct NamedDeclaration<M> {
  pub public: bool,
  pub name: Node<Identifier, M>,
  pub type_params: Vec<Node<Identifier, M>>,
  pub declaration: Node<Declaration<M>, M>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration<M> {
  Class(Class<M>),
  Effect(Effect<M>),
  Function(Function<M>),
}
