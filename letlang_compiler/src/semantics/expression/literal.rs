use crate::prelude::*;
pub use super::Model;

use letlang_parser::ast::{
  Node,
  expression::Literal,
};


impl Model {
  pub fn visit_literal(&mut self, node: &Node<Literal>) -> CompilationResult<()> {
    match node.data.as_ref() {
      Literal::Atom(val) => {
        self.interner.get_or_intern(val);
        Ok(())
      },
      _ => {
        Ok(())
      }
    }
  }
}
