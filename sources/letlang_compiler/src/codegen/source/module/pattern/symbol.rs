use crate::prelude::*;
use crate::scope::*;
pub use super::Generator;

use letlang_ast::expression::*;


impl<'compiler> Generator<'compiler> {
  pub fn gen_pattern_symbol(
    &self,
    symbol: &Symbol,
    local_scope_id: usize,
  ) -> CompilationResult<String> {
    let symbol_scope = self.get_symbol_scope(
      symbol.scope(),
      local_scope_id,
    );

    let (_, symbol_kind) = symbol_scope.lookup_symbol(
      self.scope_arena,
      &symbol.name(),
    ).unwrap();

    match symbol_kind {
      SymbolKind::Variable => {
        let symbol_name = symbol.name();
        Ok(format!("SymbolPattern {{ name: \"{symbol_name}\" }}"))
      },
      SymbolKind::CallParameter { index } => {
        Ok(format!("ValuePattern {{ llval: paramval_{index}.clone() }}"))
      },
      _ => {
        todo!();
      }
    }
  }
}
