use crate::prelude::*;
pub use super::{Model, Visitor};

use letlang_parser::ast::types::*;

impl<V: Visitor> Model<V> {
  pub fn visit_typeref_val(&mut self, type_val: &TypeVal) -> CompilationResult<()> {
    self.call_visitor(type_val)?;

    self.visit_literal(&type_val.val)?;

    Ok(())
  }

  pub fn visit_typeref_name(&mut self, type_name: &TypeName) -> CompilationResult<()> {
    self.call_visitor(type_name)?;

    self.visit_symbol(&type_name.symbol)?;

    for type_param in type_name.type_params.iter() {
      self.visit_typeref(type_param)?;
    }

    Ok(())
  }
}
