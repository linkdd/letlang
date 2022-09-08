use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};


impl<'compiler> Generator<'compiler> {
  pub fn gen_pattern_value(
    &self,
    data: &Node<Literal>,
  ) -> CompilationResult<String> {
    let value_code = self.gen_literal(data)?;
    Ok(format!("ValuePattern {{ llval: {value_code} }}"))
  }
}
