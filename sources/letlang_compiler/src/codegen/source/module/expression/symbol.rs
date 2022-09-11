use crate::prelude::*;
use crate::scope::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};


impl<'compiler> Generator<'compiler> {
  pub fn gen_symbol(
    &self,
    location: &LocationInfo,
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
        Ok(format!("helpers::assert_defined(&co, context.clone(), &mut locals, \"{symbol_name}\").await"))
      },
      SymbolKind::Function { .. } => {
        match symbol.scope() {
          None => {
            let symbol_name = symbol.name();
            Ok(format!("symbol_{symbol_name}::func_{symbol_name} {{}}"))
          },
          Some(module_scope) => {
            let crate_name = format!("lldep_{}", module_scope.replace("::", "_"));
            let symbol_name = symbol.name();

            Ok(format!("{crate_name}::symbol_{symbol_name}::func_{symbol_name} {{}}"))
          }
        }
      },
      SymbolKind::ConsParameter => {
        todo!();
      },
      SymbolKind::CallParameter { index } => {
        Ok(format!("locals.lookup_symbol(\"$param${index}\").unwrap().clone()"))
      },
      _ => {
        unreachable!("\
          Semantic validation of the AST should have proven \
          that this kind of symbol was not valid in this context\
        ");
      }
    }
  }
}
