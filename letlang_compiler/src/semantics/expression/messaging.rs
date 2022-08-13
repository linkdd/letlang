use crate::prelude::*;
pub use super::{Model, Visitor};

use letlang_parser::ast::expression::*;


impl<V: Visitor> Model<V> {
  pub fn visit_receive(&mut self, block: &Receive) -> CompilationResult<()> {
    self.call_visitor(block)?;

    for (pattern, body) in block.cases.iter() {
      self.visit_pattern(pattern)?;

      for statement in body.iter() {
        self.visit_proposition(statement)?;
      }
    }

    if let Some((timeout, body)) = &block.after {
      self.visit_expression(timeout)?;

      for statement in body.iter() {
        self.visit_proposition(statement)?;
      }
    }

    Ok(())
  }
}
