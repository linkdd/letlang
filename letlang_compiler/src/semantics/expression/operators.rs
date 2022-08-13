use crate::prelude::*;
pub use super::{Model, Visitor};

use letlang_parser::ast::expression::*;


impl<V: Visitor> Model<V> {
  pub fn visit_generic_resolve(&mut self, op: &GenericResolve) -> CompilationResult<()> {
    self.call_visitor(op)?;

    self.visit_expression(&op.symbol)?;

    for type_param in op.type_params.iter() {
      self.visit_typeref(type_param)?;
    }

    Ok(())
  }

  pub fn visit_member_access(&mut self, op: &MemberAccess) -> CompilationResult<()> {
    self.call_visitor(op)?;

    self.visit_expression(&op.lhs)?;

    Ok(())
  }

  pub fn visit_typecheck(&mut self, op: &TypeCheck) -> CompilationResult<()> {
    self.call_visitor(op)?;

    self.visit_expression(&op.lhs)?;
    self.visit_typeref(&op.rhs)?;

    Ok(())
  }

  pub fn visit_unary_operation(&mut self, op: &UnaryOperation) -> CompilationResult<()> {
    self.call_visitor(op)?;

    self.visit_expression(&op.expr)?;

    Ok(())
  }

  pub fn visit_binary_operation(&mut self, op: &BinaryOperation) -> CompilationResult<()> {
    self.call_visitor(op)?;

    self.visit_expression(&op.lhs)?;
    self.visit_expression(&op.rhs)?;

    Ok(())
  }

  pub fn visit_pattern_match(&mut self, op: &PatternMatch) -> CompilationResult<()> {
    self.call_visitor(op)?;

    self.visit_pattern(&op.lhs)?;
    self.visit_expression(&op.rhs)?;

    Ok(())
  }
}
