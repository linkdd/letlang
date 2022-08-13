use crate::prelude::*;
pub use super::{Model, Visitor};

use letlang_parser::ast::types::*;

impl<V: Visitor> Model<V> {
  pub fn visit_typeref_oneof(&mut self, sum_type: &OneOfType) -> CompilationResult<()> {
    self.call_visitor(sum_type)?;

    for typeref in sum_type.typerefs.iter() {
      self.visit_typeref(typeref)?;
    }

    Ok(())
  }

  pub fn visit_typeref_allof(&mut self, sum_type: &AllOfType) -> CompilationResult<()> {
    self.call_visitor(sum_type)?;

    for typeref in sum_type.typerefs.iter() {
      self.visit_typeref(typeref)?;
    }

    Ok(())
  }

  pub fn visit_typeref_not(&mut self, sum_type: &NotType) -> CompilationResult<()> {
    self.call_visitor(sum_type)?;

    self.visit_typeref(&sum_type.typeref)?;

    Ok(())
  }
}
