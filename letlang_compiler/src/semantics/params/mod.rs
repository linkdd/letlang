use crate::prelude::*;
pub use super::{Model, Visitor};

use letlang_parser::ast::{
  Node,
  params::{
    TypeParam,
    ConsParam,
    CallParam,
  },
};


impl<V: Visitor> Model<V> {
  pub fn visit_type_param(&mut self, node: &Node<TypeParam>) -> CompilationResult<()> {
    Self::locate_error(
      self.call_visitor(node.data.as_ref()),
      &node.location
    )
  }

  pub fn visit_cons_param(&mut self, node: &Node<ConsParam>) -> CompilationResult<()> {
    Self::locate_error(
      self.call_visitor(node.data.as_ref()),
      &node.location
    )?;

    self.visit_typeref(&node.data.param_type)?;

    Ok(())
  }

  pub fn visit_call_param(&mut self, node: &Node<CallParam>) -> CompilationResult<()> {
    Self::locate_error(
      self.call_visitor(node.data.as_ref()),
      &node.location
    )?;

    self.visit_typeref(&node.data.param_type)?;

    Ok(())
  }
}
