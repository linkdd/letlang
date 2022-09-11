use crate::prelude::*;

use super::{
  Model,
  super::scope::SymbolKind,
};

use letlang_ast::{
  *,
  expression::*,
};

impl<'compiler> Model<'compiler> {
  pub fn visit_pattern(&mut self, node: &mut Node<Pattern>) -> CompilationResult<()> {
    let attrs = node.attrs.as_ref().unwrap();

    match node.data.as_mut() {
      Pattern::Assign(symbol) => {
        if symbol.scope().is_some() {
          unreachable!("\
            The parser does not allow scoped symbols in this context\
          ");
        }

        let scope = self.scope_arena.get_scope(attrs.scope_id);

        scope.register_symbol(
          symbol.to_string(),
          false,
          SymbolKind::Variable,
        ).unwrap();

        Ok(())
      },
      Pattern::Value(expr) => {
        expr.attrs = Some(ExpressionAttributes { scope_id: attrs.scope_id });
        self.visit_expression(expr)
      },
      Pattern::Literal(literal_node) => {
        literal_node.attrs = Some(());
        self.visit_literal(literal_node)
      },
      Pattern::Tuple(data) => {
        for pattern in data.members.iter_mut() {
          pattern.attrs = Some(PatternAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_pattern(pattern)?;
        }

        Ok(())
      },
      Pattern::Struct(data) => {
        let mut local_name_scope = vec![];

        for (member_name, member_pattern) in data.members.iter_mut() {
          if local_name_scope.iter_mut().any(|name| name == member_name) {
            Err(CompilationError::new_located(
              &node.location,
              format!("duplicated struct member '{}'", member_name.clone()),
            ))?;
          }
          else {
            local_name_scope.push(member_name.clone());
          }

          member_pattern.attrs = Some(PatternAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_pattern(member_pattern)?;
        }

        Ok(())
      },
      Pattern::List(data) => {
        for item_pattern in data.items.iter_mut() {
          item_pattern.attrs = Some(PatternAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_pattern(item_pattern)?;
        }

        Ok(())
      },
      Pattern::ListHeadTail(data) => {
        data.head.attrs = Some(PatternAttributes {
          scope_id: attrs.scope_id,
        });
        data.tail.attrs = Some(PatternAttributes {
          scope_id: attrs.scope_id,
        });
        self.visit_pattern(&mut data.head)?;
        self.visit_pattern(&mut data.tail)?;
        Ok(())
      }
    }
  }
}
