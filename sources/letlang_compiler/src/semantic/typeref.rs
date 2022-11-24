use crate::prelude::*;

use super::{Model, super::scope::SymbolKind};

use letlang_ast::{
  *,
  types::*,
};

impl<'compiler> Model<'compiler> {
  pub fn visit_typeref(&mut self, node: &mut Node<TypeRef>) -> CompilationResult<()> {
    let attrs = node.attrs.as_ref().unwrap();

    match node.data.as_mut() {
      TypeRef::Value(data) => {
        self.visit_literal(&mut data.val)
      },
      TypeRef::TypeName(data) => {
        let sym = self.visit_symbol(&mut data.symbol, attrs.scope_id)?;

        match sym {
          None => {
            return Err(CompilationError::new_located(
              &node.location,
              format!("Unknown symbol '{}'", data.symbol.to_string()),
            ))
          },
          Some(sym_kind) => {
            match sym_kind {
              SymbolKind::Class { type_param_count, .. } => {
                if type_param_count != data.type_params.len() {
                  return Err(CompilationError::new_located(
                    &node.location,
                    format!(
                      "Class '{}' expects {} type parameters, got {}",
                      data.symbol.to_string(),
                      type_param_count,
                      data.type_params.len(),
                    ),
                  ));
                }
              },
              SymbolKind::TypeParameter { .. } => {
                if data.type_params.len() > 0 {
                  return Err(CompilationError::new_located(
                    &node.location,
                    format!("Type parameter {} cannot be generic", data.symbol.to_string()),
                  ));
                }
              },
              _ => {
                return Err(CompilationError::new_located(
                  &node.location,
                  format!("Invalid symbol '{}' in this context", data.symbol.to_string()),
                ));
              }
            }
          }
        }

        for type_param in data.type_params.iter_mut() {
          type_param.attrs = Some(TypeRefAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_typeref(type_param)?;
        }

        Ok(())
      },
      TypeRef::StructDefinition(data) => {
        let mut local_name_scope = vec![];

        for (member_name, member_type) in data.members.iter_mut() {
          if local_name_scope.iter_mut().any(|name| name == member_name) {
            return Err(CompilationError::new_located(
              &node.location,
              format!("duplicated struct member '{}'", member_name.clone()),
            ));
          }
          else {
            local_name_scope.push(member_name.clone());
          }

          member_type.attrs = Some(TypeRefAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_typeref(member_type)?;
        }

        Ok(())
      },
      TypeRef::TupleDefinition(data) => {
        for member_type in data.members.iter_mut() {
          member_type.attrs = Some(TypeRefAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_typeref(member_type)?;
        }

        Ok(())
      },
      TypeRef::FunctionSignature(data) => {
        for param in data.params.iter_mut() {
          param.attrs = Some(TypeRefAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_typeref(param)?;
        }

        data.return_type.attrs = Some(TypeRefAttributes {
          scope_id: attrs.scope_id,
        });
        self.visit_typeref(&mut data.return_type)?;

        Ok(())
      },
      TypeRef::OneOf(data) => {
        for subtype in data.typerefs.iter_mut() {
          subtype.attrs = Some(TypeRefAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_typeref(subtype)?;
        }

        Ok(())
      },
      TypeRef::AllOf(data) => {
        for subtype in data.typerefs.iter_mut() {
          subtype.attrs = Some(TypeRefAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_typeref(subtype)?;
        }

        Ok(())
      },
      TypeRef::Not(data) => {
        data.typeref.attrs = Some(TypeRefAttributes {
          scope_id: attrs.scope_id,
        });
        self.visit_typeref(&mut data.typeref)
      }
    }
  }
}
