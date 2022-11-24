use crate::prelude::*;
pub use super::Generator;

use letlang_ast::expression::*;


impl<'compiler> Generator<'compiler> {
  pub fn gen_pattern_assign(&self, symbol: &Symbol) -> CompilationResult<String> {
    let symbol_name = symbol.name();
    Ok(format!("SymbolPattern {{ name: \"{symbol_name}\" }}"))
  }
}
