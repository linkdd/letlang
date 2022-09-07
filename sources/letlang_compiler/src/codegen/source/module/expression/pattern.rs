use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};


impl<'compiler> Generator<'compiler> {
  pub fn gen_pattern_match(
    &self,
    location: &LocationInfo,
    data: &PatternMatch,
  ) -> CompilationResult<String> {
    let expr_code = self.gen_expression(&data.rhs)?;
    todo!();
  }
}
