use crate::prelude::*;
pub use super::{Model, Visitor};

use letlang_parser::ast::expression::*;


impl<V: Visitor> Model<V> {
  pub fn visit_symbol(&mut self, sym: &Symbol) -> CompilationResult<()> {
    self.call_visitor(sym)
  }
}
