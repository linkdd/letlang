use crate::prelude::*;
pub use super::{Model, Visitor};

use letlang_parser::ast::{
  Node,
  expression::Literal,
};


impl<V: Visitor> Model<V> {
  pub fn visit_literal(&mut self, node: &Node<Literal>) -> CompilationResult<()> {
    Self::locate_error(
      self.call_visitor(node.data.as_ref()),
      &node.location,
    )
  }
}
