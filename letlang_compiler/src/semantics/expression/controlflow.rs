use crate::prelude::*;
pub use super::{Model, Visitor};

use letlang_parser::ast::expression::*;


impl<V: Visitor> Model<V> {
  pub fn visit_flow_loop(&mut self, controlflow: &Loop) -> CompilationResult<()> {
    self.call_visitor(controlflow)?;

    for statement in controlflow.body.iter() {
      self.visit_proposition(statement)?;
    }

    Ok(())
  }

  pub fn visit_flow_break(&mut self, controlflow: &Break) -> CompilationResult<()> {
    self.call_visitor(controlflow)?;

    self.visit_expression(&controlflow.value)?;

    Ok(())
  }

  pub fn visit_flow_match(&mut self, controlflow: &FlowMatch) -> CompilationResult<()> {
    self.call_visitor(controlflow)?;

    self.visit_expression(&controlflow.expr)?;

    for (pattern, body) in controlflow.cases.iter() {
      self.visit_pattern(pattern)?;

      for statement in body.iter() {
        self.visit_proposition(statement)?;
      }
    }

    Ok(())
  }

  pub fn visit_flow_conditional(&mut self, controlflow: &FlowConditional) -> CompilationResult<()> {
    self.call_visitor(controlflow)?;

    for (condition, body) in controlflow.cases.iter() {
      self.visit_expression(condition)?;

      for statement in body.iter() {
        self.visit_proposition(statement)?;
      }
    }

    for statement in controlflow.else_case.iter() {
      self.visit_proposition(statement)?;
    }

    Ok(())
  }
}
