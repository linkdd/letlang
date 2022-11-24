use crate::prelude::*;
pub use super::Generator;

use letlang_ast::types::*;


impl<'compiler> Generator<'compiler> {
  pub fn gen_typeref_value(&self, data: &TypeVal) -> CompilationResult<String> {
    let value_code = self.gen_literal(&data.val)?;
    Ok(format!("ValueType {{ llval: {} }}", value_code))
  }
}
