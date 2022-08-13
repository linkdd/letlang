use crate::prelude::*;
pub use super::{Model, Visitor};

use letlang_parser::ast::types::*;

impl<V: Visitor> Model<V> {
  pub fn visit_typeref_struct(&mut self, def: &StructDefinition) -> CompilationResult<()> {
    self.call_visitor(def)?;

    for (_, member_type) in def.members.iter() {
      self.visit_typeref(member_type)?;
    }

    Ok(())
  }

  pub fn visit_typeref_tuple(&mut self, def: &TupleDefinition) -> CompilationResult<()> {
    self.call_visitor(def)?;

    for member_type in def.members.iter() {
      self.visit_typeref(member_type)?;
    }

    Ok(())
  }
}
