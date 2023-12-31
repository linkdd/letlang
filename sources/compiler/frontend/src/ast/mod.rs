use nonempty::NonEmpty;
use ast_core::*;

mod import;
mod decls;
mod typ;
mod expr;
mod pat;

pub use self::{
  import::*,
  decls::*,
  typ::*,
  expr::*,
  pat::*,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AST<M>(pub Node<Module<M>, M>);

#[derive(Debug, Clone, PartialEq)]
pub struct Module<M> {
  pub path: NonEmpty<Node<Identifier, M>>,
  pub imports: Vec<Node<Import<M>, M>>,
  pub declarations: Vec<Node<NamedDeclaration<M>, M>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);
