use crate::prelude::*;
use super::Model;

use letlang_parser::ast::{
  Node,
  statement::Proposition,
  types::TypeRef,
  expression::Expression,
};


impl Model {
  pub fn visit_proposition(&mut self, node: &Node<Proposition>) -> CompilationResult<()> {
    match node.data.as_ref() {
      Proposition::Constraint { symbol_name, symbol_type, checks } => {
        self.visit_proposition_constraint(symbol_name, symbol_type, checks)
      },
      Proposition::Evaluation(expr) => {
        self.visit_expression(expr)
      }
    }
  }

  fn visit_proposition_constraint(
    &mut self,
    symbol_name: &String,
    symbol_type: &Node<TypeRef>,
    checks: &Vec<Node<Expression>>,
  ) -> CompilationResult<()> {
    for check in checks.iter() {
      self.visit_expression(check)?;
    }

    todo!();
  }
}