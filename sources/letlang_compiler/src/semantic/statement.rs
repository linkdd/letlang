use crate::prelude::*;

use super::{
  Model,
  super::scope::SymbolKind,
};

use letlang_ast::{
  *,
  statement::*,
  types::*,
  params::*,
};

impl<'compiler> Model<'compiler> {
  pub fn visit_statement(&mut self, node: &mut Node<Statement>) -> CompilationResult<Option<String>> {
    let attrs = node.attrs.as_ref().unwrap();

    match node.data.as_mut() {
      Statement::Import(data) => {
        let unit_key = format!("lldep_{}", data.path.join("_"));

        self.visit_unit_key(unit_key.clone())?;

        let unit_dep = self.unit_registry.get(&unit_key).unwrap().borrow();
        let unit_dep_attrs = unit_dep.attrs.as_ref().unwrap();
        let scope = self.scope_arena.get_scope(attrs.scope_id);

        let path = match data.alias.as_ref() {
          None => {
            scope.register_symbol(
              data.path.join("::"),
              false,
              SymbolKind::Module { scope_id: unit_dep_attrs.scope_id },
            );
            data.path.clone()
          },
          Some(alias) => {
            scope.register_symbol(
              alias.clone(),
              false,
              SymbolKind::Module { scope_id: unit_dep_attrs.scope_id },
            );
            vec![alias.clone()]
          }
        };

        Ok(Some(unit_key))
      },
      Statement::EffectDecl(data) => {
        let scope = self.scope_arena.get_scope(attrs.scope_id);
        scope.register_symbol(
          data.symbol_name.clone(),
          data.public,
          SymbolKind::Effect {
            call_param_count: data.call_params.len(),
          }
        );

        let effect_scope_id = self.scope_arena.new_scope(attrs.scope_id);

        for (index, call_param_node) in data.call_params.iter_mut().enumerate() {
          call_param_node.attrs = Some(CallParamAttributes {
            scope_id: effect_scope_id,
            index
          });

          self.visit_param_call(call_param_node)?;
        }

        data.return_type.attrs = Some(TypeRefAttributes {
          scope_id: effect_scope_id,
        });

        self.visit_typeref(&mut data.return_type)?;

        Ok(None)
      },
      Statement::ClassDecl(data) => {
        let scope = self.scope_arena.get_scope(attrs.scope_id);
        scope.register_symbol(
          data.symbol_name.clone(),
          data.public,
          SymbolKind::Class {
            type_param_count: data.type_params.len(),
            builtin: false,
          }
        );

        let class_scope_id = self.scope_arena.new_scope(attrs.scope_id);

        for (index, type_param_node) in data.type_params.iter_mut().enumerate() {
          type_param_node.attrs = Some(TypeParamAttributes {
            scope_id: class_scope_id,
            index
          });
          self.visit_param_type(type_param_node)?;
        }

        data.cons_param.attrs = Some(ConsParamAttributes {
          scope_id: class_scope_id,
        });
        self.visit_param_cons(&mut data.cons_param)?;

        for proposition_node in data.constraints.iter_mut() {
          proposition_node.attrs = Some(PropositionAttributes {
            scope_id: class_scope_id,
          });

          self.visit_proposition(proposition_node)?;
        }

        Ok(None)
      },
      Statement::FuncDecl(data) => {
        let scope = self.scope_arena.get_scope(attrs.scope_id);
        scope.register_symbol(
          data.symbol_name.clone(),
          data.public,
          SymbolKind::Function {
            type_param_count: data.type_params.len(),
            call_param_count: data.call_params.len(),
          }
        );

        let func_scope_id = self.scope_arena.new_scope(attrs.scope_id);

        for (index, type_param_node) in data.type_params.iter_mut().enumerate() {
          type_param_node.attrs = Some(TypeParamAttributes {
            scope_id: func_scope_id,
            index
          });
          self.visit_param_type(type_param_node)?;
        }

        for (index, call_param_node) in data.call_params.iter_mut().enumerate() {
          call_param_node.attrs = Some(CallParamAttributes {
            scope_id: func_scope_id,
            index
          });

          self.visit_param_call(call_param_node)?;
        }

        data.return_type.attrs = Some(TypeRefAttributes {
          scope_id: func_scope_id,
        });

        self.visit_typeref(&mut data.return_type)?;

        for proposition_node in data.body.iter_mut() {
          proposition_node.attrs = Some(PropositionAttributes {
            scope_id: func_scope_id,
          });

          self.visit_proposition(proposition_node)?;
        }

        Ok(None)
      }
    }
  }
}
