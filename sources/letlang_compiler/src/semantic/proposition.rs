use crate::prelude::*;

use super::{Model, super::scope::SymbolKind};

use letlang_ast::{
  *,
  statement::*,
  types::*,
  expression::*,
};

impl<'compiler> Model<'compiler> {
  pub fn visit_proposition(&mut self, node: &mut Node<Proposition>) -> CompilationResult<()> {
    let attrs = node.attrs.as_ref().unwrap();

    match node.data.as_mut() {
      Proposition::Constraint(data) => {
        let scope = self.scope_arena.get_scope(attrs.scope_id);

        let sym = scope.lookup_symbol(
          &self.scope_arena,
          &data.symbol_name
        );

        match sym {
          None => {
            scope.register_symbol(
              data.symbol_name.clone(),
              false,
              SymbolKind::Variable
            ).unwrap();
          },
          Some((_, sym_type)) => {
            match sym_type {
              SymbolKind::Variable => {},
              SymbolKind::Function { .. } => {},
              SymbolKind::ConsParameter => {},
              SymbolKind::CallParameter { .. } => {},
              _ => {
                return Err(CompilationError::new_located(
                  &node.location,
                  format!("Symbol '{}' is invalid in this context", data.symbol_name.clone()),
                ));
              }
            }
          }
        }

        data.symbol_type.attrs = Some(TypeRefAttributes {
          scope_id: attrs.scope_id,
        });
        self.visit_typeref(&mut data.symbol_type)?;

        let constraint_scope_id = self.scope_arena.new_scope(attrs.scope_id);

        for expression in data.checks.iter_mut() {
          expression.attrs = Some(ExpressionAttributes {
            scope_id: constraint_scope_id,
          });
          self.visit_expression(expression)?;
        }

        Ok(())
      },
      Proposition::Evaluation(expr) => {
        expr.attrs = Some(ExpressionAttributes {
          scope_id: attrs.scope_id,
        });
        self.visit_expression(expr)
      }
    }
  }
}
