use crate::prelude::*;

use super::{
  Model,
  super::scope::SymbolKind,
};

use letlang_ast::{
  *,
  unit::*,
  statement::*,
};

impl<'compiler> Model<'compiler> {
  pub fn visit_unit_key(&mut self, unit_key: String) -> CompilationResult<()> {
    match self.unit_registry.get(&unit_key) {
      None => {
        Err(CompilationError::new(
          format!("Unit '{}' not found", unit_key)
        ))
      },
      Some(node_cell) => {
        let mut node = node_cell.borrow_mut();

        match node.attrs {
          None => {
            let scope_id = self.scope_arena.new_root_scope();
            let scope = self.scope_arena.get_scope(scope_id);

            scope.register_symbol(
              "boolean".to_string(),
              false,
              SymbolKind::Class { type_param_count: 0, builtin: true },
            ).unwrap();

            scope.register_symbol(
              "number".to_string(),
              false,
              SymbolKind::Class { type_param_count: 0, builtin: true },
            ).unwrap();

            scope.register_symbol(
              "int".to_string(),
              false,
              SymbolKind::Class { type_param_count: 0, builtin: true },
            ).unwrap();

            scope.register_symbol(
              "string".to_string(),
              false,
              SymbolKind::Class { type_param_count: 0, builtin: true },
            ).unwrap();

            scope.register_symbol(
              "atom".to_string(),
              false,
              SymbolKind::Class { type_param_count: 0, builtin: true },
            ).unwrap();

            scope.register_symbol(
              "pid".to_string(),
              false,
              SymbolKind::Class { type_param_count: 0, builtin: true },
            ).unwrap();

            scope.register_symbol(
              "list".to_string(),
              false,
              SymbolKind::Class { type_param_count: 1, builtin: true },
            ).unwrap();

            node.attrs = Some(UnitAttributes {
              scope_id,
              unit_key: unit_key.clone(),
              dependencies: vec![],
            });

            self.visit_unit(&mut node)
          },
          Some(_) => {
            Ok(())
          }
        }
      }
    }
  }

  pub fn visit_unit(&mut self, node: &mut Node<Unit>) -> CompilationResult<()> {
    let attrs = node.attrs.as_mut().unwrap();

    for statement_node in node.data.statements.iter_mut() {
      statement_node.attrs = Some(StatementAttributes {
        scope_id: attrs.scope_id,
      });

      match self.visit_statement(statement_node)? {
        None => {},
        Some(dep) => {
          attrs.dependencies.push(dep);
        }
      };
    }

    Ok(())
  }
}
