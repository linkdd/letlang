use crate::prelude::*;

use super::{Model, super::scope::SymbolKind};

use letlang_ast::{
  expression::*,
};

impl<'compiler> Model<'compiler> {
  pub fn visit_symbol(&mut self, sym: &Symbol, scope_id: usize) -> CompilationResult<Option<SymbolKind>> {
    let scope = self.scope_arena.get_scope(scope_id);

    let sym_scope = match sym.scope() {
      None => {
        scope
      },
      Some(scope_name) => {
        let module_sym = scope.lookup_symbol(
          &self.scope_arena,
          &scope_name,
        );

        match module_sym {
          None => {
            return Ok(None);
          },
          Some((_, mod_sym_kind)) => {
            if let SymbolKind::Module { scope_id: mod_scope_id } = mod_sym_kind {
              self.scope_arena.get_scope(mod_scope_id)
            }
            else {
              return Ok(None)
            }
          }
        }
      }
    };

    let sym_kind = sym_scope
      .lookup_symbol(&self.scope_arena, &sym.name())
      .and_then(|(public, sym_kind)| {
        if sym.scope().is_none() || public {
          Some(sym_kind)
        }
        else {
          None
        }
      });

    Ok(sym_kind)
  }
}
