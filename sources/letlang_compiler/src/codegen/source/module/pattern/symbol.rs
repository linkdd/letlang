use crate::prelude::*;
pub use super::Generator;

use letlang_ast::expression::*;


impl<'compiler> Generator<'compiler> {
  pub fn gen_pattern_symbol(
    &self,
    sym: &Symbol,
  ) -> CompilationResult<String> {
    let symbol_name = sym.name();
    Ok(format!("SymbolPattern {{ name: \"{symbol_name}\" }}"))
  }
}
