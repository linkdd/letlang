use nonempty::NonEmpty;
use ast_core::*;

use crate::ast::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub enum Import<M> {
  Symbol {
    path: NonEmpty<Node<Identifier, M>>,
    symbols: NonEmpty<(Node<Identifier, M>, Option<Node<Identifier, M>>)>,
  },
  Module {
    path: NonEmpty<Node<Identifier, M>>,
    alias: Option<Node<Identifier, M>>,
  },
}
