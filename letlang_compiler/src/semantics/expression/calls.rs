use crate::prelude::*;
pub use super::{Model, Visitor};

use letlang_parser::ast::expression::*;


impl<V: Visitor> Model<V> {
  pub fn visit_effect_call(&mut self, call: &EffectCall) -> CompilationResult<()> {
    self.call_visitor(call)?;

    self.visit_symbol(&call.effect_name)?;

    for param in call.params.iter() {
      self.visit_expression(param)?;
    }

    Ok(())
  }

  pub fn visit_function_call(&mut self, call: &FunctionCall) -> CompilationResult<()> {
    self.call_visitor(call)?;

    self.visit_expression(&call.func)?;

    for param in call.params.iter() {
      self.visit_expression(param)?;
    }

    Ok(())
  }

  pub fn visit_spawn_call(&mut self, call: &SpawnCall) -> CompilationResult<()> {
    self.call_visitor(call)?;

    self.visit_expression(&call.func)?;

    for param in call.params.iter() {
      self.visit_expression(param)?;
    }

    Ok(())
  }
}
