use crate::prelude::*;
pub use super::{Model, Visitor};

use letlang_parser::ast::expression::*;


impl<V: Visitor> Model<V> {
  pub fn visit_structure(&mut self, structure: &Structure) -> CompilationResult<()> {
    self.call_visitor(structure)?;

    for (member_name, member_val) in structure.members.iter() {
      self.visit_expression(member_val)?;
    }

    Ok(())
  }

  pub fn visit_tuple(&mut self, tuple: &Tuple) -> CompilationResult<()> {
    self.call_visitor(tuple)?;

    for member in tuple.members.iter() {
      self.visit_expression(member)?;
    }

    Ok(())
  }

  pub fn visit_list(&mut self, list: &List) -> CompilationResult<()> {
    self.call_visitor(list)?;

    for item in list.items.iter() {
      self.visit_expression(item)?;
    }

    Ok(())
  }
}
