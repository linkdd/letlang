use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};

use askama::Template;


#[derive(Template)]
#[template(path = "expression/pattern_match.rs.j2", escape = "none")]
struct PatternMatchTemplate {
  expr_code: String,
  pattern_code: String,
}

impl<'compiler> Generator<'compiler> {
  pub fn gen_pattern_match(
    &self,
    location: &LocationInfo,
    data: &PatternMatch,
  ) -> CompilationResult<String> {
    let expr_code = self.gen_expression(&data.rhs)?;
    let pattern_code = self.gen_pattern(&data.lhs)?;

    let context = PatternMatchTemplate {
      expr_code,
      pattern_code,
    };

    let source_code = context.render().map_err(|e| {
      CompilationError::new_located(
        location,
        format!("Could not generate pattern matching source: {}", e),
      )
    })?;

    Ok(source_code)
  }
}
