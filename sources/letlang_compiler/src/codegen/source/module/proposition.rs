use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  statement::*,
};

use askama::Template;


#[derive(Template)]
#[template(path = "statement/proposition_constraint.rs.j2", escape = "none")]
struct ConstraintTemplate {
  symbol: String,
  type_code: String,
  checks: Vec<String>,
}


impl<'compiler> Generator<'compiler> {
  pub fn gen_proposition(&self, node: &Node<Proposition>) -> CompilationResult<String> {
    match node.data.as_ref() {
      Proposition::Constraint(constraint) => {
        let type_code = self.gen_typeref(&constraint.symbol_type)?;
        let mut checks = vec![];

        for check_node in constraint.checks.iter() {
          let check_code = self.gen_expression(check_node)?;
          checks.push(check_code);
        }

        let context = ConstraintTemplate {
          symbol: constraint.symbol_name.clone(),
          type_code,
          checks,
        };

        let source_code = context.render().map_err(|e| {
          CompilationError::new_located(
            &node.location,
            format!("Could not generate constraint source: {}", e),
          )
        })?;

        Ok(source_code)
      },
      Proposition::Evaluation(expr) => {
        self.gen_expression(expr)
      }
    }
  }
}
