use crate::prelude::*;

use super::{
  Model,
  super::scope::SymbolKind,
};

use letlang_ast::{
  *,
  params::*,
  types::*,
};


impl<'compiler> Model<'compiler> {
  pub fn visit_param_type(&mut self, node: &mut Node<TypeParam>) -> CompilationResult<()> {
    let attrs = node.attrs.as_ref().unwrap();
    let scope = self.scope_arena.get_scope(attrs.scope_id);

    scope
      .register_symbol(
        node.data.name.clone(),
        false,
        SymbolKind::TypeParameter { index: attrs.index }
      )
      .map_err(|()| {
        CompilationError::new_located(
          &node.location,
          format!(
            "Symbol '{}' already exists",
            node.data.name.clone(),
          ),
        )
      })?;

    Ok(())
  }

  pub fn visit_param_cons(&mut self, node: &mut Node<ConsParam>) -> CompilationResult<()> {
    let attrs = node.attrs.as_ref().unwrap();
    let scope = self.scope_arena.get_scope(attrs.scope_id);

    scope
      .register_symbol(
        node.data.param_name.clone(),
        false,
        SymbolKind::ConsParameter,
      )
      .map_err(|()| {
        CompilationError::new_located(
          &node.location,
          format!(
            "Symbol '{}' already exists",
            node.data.param_name.clone(),
          ),
        )
      })?;

    node.data.param_type.attrs = Some(TypeRefAttributes {
      scope_id: attrs.scope_id,
    });
    self.visit_typeref(&mut node.data.param_type)
  }

  pub fn visit_param_call(&mut self, node: &mut Node<CallParam>) -> CompilationResult<()> {
    let attrs = node.attrs.as_ref().unwrap();
    let scope = self.scope_arena.get_scope(attrs.scope_id);

    scope
      .register_symbol(
        node.data.param_name.clone(),
        false,
        SymbolKind::CallParameter { index: attrs.index }
      )
      .map_err(|()| {
        CompilationError::new_located(
          &node.location,
          format!("Symbol '{}' already exists", node.data.param_name.clone()),
        )
      })?;

    node.data.param_type.attrs = Some(TypeRefAttributes {
      scope_id: attrs.scope_id,
    });
    self.visit_typeref(&mut node.data.param_type)
  }
}
