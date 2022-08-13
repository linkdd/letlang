use crate::prelude::*;
use super::{Model, Visitor};

use letlang_parser::ast::{
  Node,
  statement::{Proposition, Constraint},
};


impl<V: Visitor> Model<V> {
  pub fn visit_proposition(&mut self, node: &Node<Proposition>) -> CompilationResult<()> {
    let result = match node.data.as_ref() {
      Proposition::Constraint(constraint) => {
        self.visit_proposition_constraint(constraint)
      },
      Proposition::Evaluation(expr) => {
        self.visit_expression(expr)
      }
    };

    Self::locate_error(result, &node.location)
  }

  fn visit_proposition_constraint(&mut self, constraint: &Constraint) -> CompilationResult<()> {
    self.call_visitor(constraint)?;

    self.visit_typeref(&constraint.symbol_type)?;

    for check in constraint.checks.iter() {
      self.visit_expression(check)?;
    }

    Ok(())
  }
}