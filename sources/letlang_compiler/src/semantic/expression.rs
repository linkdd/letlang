use crate::prelude::*;

use super::{
  Model,
  super::scope::SymbolKind,
};

use letlang_ast::{
  *,
  statement::*,
  types::*,
  expression::*,
};

impl<'compiler> Model<'compiler> {
  pub fn visit_expression(&mut self, node: &mut Node<Expression>) -> CompilationResult<()> {
    let attrs = node.attrs.as_ref().unwrap();
    let scope = self.scope_arena.get_scope(attrs.scope_id);

    match node.data.as_mut() {
      Expression::Symbol(symbol) => {
        let sym = self.visit_symbol(symbol, attrs.scope_id)?;

        match sym {
          None => {
            return Err(CompilationError::new_located(
              &node.location,
              format!("Unknown symbol '{}'", symbol.to_string()),
            ));
          },
          Some(sym_kind) => {
            match sym_kind {
              SymbolKind::Variable => {},
              SymbolKind::Function { type_param_count: 0, .. } => {},
              SymbolKind::ConsParameter => {},
              SymbolKind::CallParameter { .. } => {},
              _ => {
                return Err(CompilationError::new_located(
                  &node.location,
                  format!("Invalid symbol '{}' in this context", symbol.to_string())
                ));
              }
            }
          }
        }

        Ok(())
      },
      Expression::Literal(literal_node) => {
        literal_node.attrs = Some(());
        self.visit_literal(literal_node)
      },
      Expression::Structure(data) => {
        let mut local_name_scope = vec![];

        for (member_name, member_value) in data.members.iter_mut() {
          if local_name_scope.iter_mut().any(|name| name == member_name) {
            return Err(CompilationError::new_located(
              &node.location,
              format!("duplicated struct member '{}'", member_name.clone()),
            ));
          }
          else {
            local_name_scope.push(member_name.clone());
          }

          member_value.attrs = Some(ExpressionAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_expression(member_value)?;
        }

        Ok(())
      },
      Expression::Tuple(data) => {
        for member_value in data.members.iter_mut() {
          member_value.attrs = Some(ExpressionAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_expression(member_value)?;
        }

        Ok(())
      },
      Expression::List(data) => {
        for item_value in data.items.iter_mut() {
          item_value.attrs = Some(ExpressionAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_expression(item_value)?;
        }

        Ok(())
      },
      Expression::EffectCall(data) => {
        let sym = self.visit_symbol(&mut data.effect_name, attrs.scope_id)?;

        match sym {
          None => {
            return Err(CompilationError::new_located(
              &node.location,
              format!("Unknown symbol '{}'", data.effect_name.to_string()),
            ));
          },
          Some(sym_kind) => {
            match sym_kind {
              SymbolKind::Effect { call_param_count } => {
                if call_param_count != data.params.len() {
                  return Err(CompilationError::new_located(
                    &node.location,
                    format!(
                      "Effect '{}' expects {} call parameters, got {}",
                      data.effect_name.to_string(),
                      call_param_count,
                      data.params.len(),
                    ),
                  ));
                }
              },
              _ => {
                return Err(CompilationError::new_located(
                  &node.location,
                  format!("Invalid symbol '{}' in this context", data.effect_name.to_string()),
                ));
              },
            }
          }
        }

        for param in data.params.iter_mut() {
          param.attrs = Some(ExpressionAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_expression(param)?;
        }

        Ok(())
      },
      Expression::FunctionCall(data) => {
        data.func.attrs = Some(ExpressionAttributes {
          scope_id: attrs.scope_id,
        });
        self.visit_expression(&mut data.func)?;

        for param in data.params.iter_mut() {
          param.attrs = Some(ExpressionAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_expression(param)?;
        }

        Ok(())
      },
      Expression::SpawnCall(data) => {
        data.func.attrs = Some(ExpressionAttributes {
          scope_id: attrs.scope_id,
        });
        self.visit_expression(&mut data.func)?;

        for param in data.params.iter_mut() {
          param.attrs = Some(ExpressionAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_expression(param)?;
        }

        Ok(())
      },
      Expression::GenericResolve(data) => {
        match data.symbol.data.as_mut() {
          Expression::Symbol(symbol) => {
            let sym = self.visit_symbol(symbol, attrs.scope_id)?;

            match sym {
              None => {
                return Err(CompilationError::new_located(
                  &node.location,
                  format!("Unknown symbol '{}'", symbol.to_string()),
                ));
              },
              Some(sym_kind) => {
                match sym_kind {
                  SymbolKind::Class { type_param_count, .. } => {
                    if type_param_count != data.type_params.len() {
                      return Err(CompilationError::new_located(
                        &node.location,
                        format!(
                          "Class '{}' expects {} type parameters, got {}",
                          symbol.to_string(),
                          type_param_count,
                          data.type_params.len(),
                        ),
                      ));
                    }
                  },
                  SymbolKind::Function { type_param_count, .. } => {
                    if type_param_count != data.type_params.len() {
                      return Err(CompilationError::new_located(
                        &node.location,
                        format!(
                          "Function '{}' expects {} type parameters, got {}",
                          symbol.to_string(),
                          type_param_count,
                          data.type_params.len(),
                        ),
                      ));
                    }
                  },
                  _ => {
                    return Err(CompilationError::new_located(
                      &node.location,
                      format!("Invalid symbol '{}' in this context", symbol.to_string()),
                    ));
                  }
                }
              }
            }

            Ok(())
          },
          _ => {
            Err(CompilationError::new_located(
              &node.location,
              "generic specialization can only be performed on a symbol".to_string(),
            ))
          }
        }?;

        for type_param in data.type_params.iter_mut() {
          type_param.attrs = Some(TypeRefAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_typeref(type_param)?;
        }

        Ok(())
      },
      Expression::MemberAccess(data) => {
        data.lhs.attrs = Some(ExpressionAttributes {
          scope_id: attrs.scope_id,
        });
        self.visit_expression(&mut data.lhs)
      },
      Expression::TypeCheck(data) => {
        data.lhs.attrs = Some(ExpressionAttributes {
          scope_id: attrs.scope_id,
        });
        data.rhs.attrs = Some(TypeRefAttributes {
          scope_id: attrs.scope_id,
        });
        self.visit_expression(&mut data.lhs)?;
        self.visit_typeref(&mut data.rhs)?;
        Ok(())
      },
      Expression::UnaryOperation(data) => {
        data.expr.attrs = Some(ExpressionAttributes {
          scope_id: attrs.scope_id,
        });
        self.visit_expression(&mut data.expr)
      },
      Expression::BinaryOperation(data) => {
        data.lhs.attrs = Some(ExpressionAttributes {
          scope_id: attrs.scope_id,
        });
        data.rhs.attrs = Some(ExpressionAttributes {
          scope_id: attrs.scope_id,
        });

        match (data.lhs.data.as_mut(), data.op, data.rhs.data.as_mut()) {
          (_, "|>", Expression::FunctionCall(_)) => {
            self.visit_expression(&mut data.lhs)?;
            self.visit_expression(&mut data.rhs)?;
            Ok(())
          },
          (_, "|>", _) => {
            Err(CompilationError::new_located(
              &mut data.rhs.location,
              "pipeline operator's right operand must be a function call".to_string()
            ))
          },
          _ => {
            self.visit_expression(&mut data.lhs)?;
            self.visit_expression(&mut data.rhs)?;
            Ok(())
          }
        }
      },
      Expression::PatternMatch(data) => {
        data.lhs.attrs = Some(PatternAttributes {
          scope_id: attrs.scope_id,
        });
        data.rhs.attrs = Some(ExpressionAttributes {
          scope_id: attrs.scope_id,
        });

        self.visit_pattern(&mut data.lhs)?;
        self.visit_expression(&mut data.rhs)?;
        Ok(())
      },
      Expression::Loop(data) => {
        let loop_scope_id = self.scope_arena.new_scope(attrs.scope_id);
        let loop_scope = self.scope_arena.get_scope(loop_scope_id);

        let loop_label_symbol_key = format!("$loop${}", data.label);
        let loop_label_symbol = loop_scope.lookup_symbol(
          &self.scope_arena,
          &loop_label_symbol_key,
        );

        if loop_label_symbol.is_none() {
          loop_scope.register_symbol(
            loop_label_symbol_key,
            false,
            SymbolKind::Label,
          );
        }
        else if let Some((_, SymbolKind::Label)) = loop_label_symbol {
        }
        else {
          return Err(CompilationError::new_located(
            &node.location,
            format!("Loop label '{}' already exists", data.label),
          ));
        }

        for proposition in data.body.iter_mut() {
          proposition.attrs = Some(PropositionAttributes {
            scope_id: loop_scope_id,
          });
          self.visit_proposition(proposition)?;
        }

        Ok(())
      },
      Expression::Break(data) => {
        let loop_label_symbol_key = format!("$loop${}", data.label);
        let loop_label_symbol = scope.lookup_symbol(
          &self.scope_arena,
          &loop_label_symbol_key,
        );

        if loop_label_symbol.is_none() {
          return Err(CompilationError::new_located(
            &node.location,
            format!("Unknown loop label '{}'", data.label),
          ));
        }

        data.value.attrs = Some(ExpressionAttributes {
          scope_id: attrs.scope_id,
        });
        self.visit_expression(&mut data.value)
      },
      Expression::FlowMatch(data) => {
        data.expr.attrs = Some(ExpressionAttributes {
          scope_id: attrs.scope_id,
        });
        self.visit_expression(&mut data.expr)?;

        for (pattern, branch) in data.cases.iter_mut() {
          let branch_scope_id = self.scope_arena.new_scope(attrs.scope_id);

          pattern.attrs = Some(PatternAttributes {
            scope_id: branch_scope_id,
          });
          self.visit_pattern(pattern)?;

          for proposition in branch.iter_mut() {
            proposition.attrs = Some(PropositionAttributes {
              scope_id: branch_scope_id,
            });
            self.visit_proposition(proposition)?;
          }
        }

        Ok(())
      },
      Expression::FlowConditional(data) => {
        for (cond, branch) in data.cases.iter_mut() {
          cond.attrs = Some(ExpressionAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_expression(cond)?;

          let branch_scope_id = self.scope_arena.new_scope(attrs.scope_id);

          for proposition in branch.iter_mut() {
            proposition.attrs = Some(PropositionAttributes {
              scope_id: branch_scope_id,
            });
            self.visit_proposition(proposition)?;
          }
        }

        let else_scope_id = self.scope_arena.new_scope(attrs.scope_id);

        for proposition in data.else_case.iter_mut() {
          proposition.attrs = Some(PropositionAttributes {
            scope_id: else_scope_id,
          });
          self.visit_proposition(proposition)?;
        }

        Ok(())
      },
      Expression::Receive(data) => {
        for (pattern, branch) in data.cases.iter_mut() {
          let branch_scope_id = self.scope_arena.new_scope(attrs.scope_id);

          pattern.attrs = Some(PatternAttributes {
            scope_id: branch_scope_id,
          });
          self.visit_pattern(pattern)?;

          for proposition in branch.iter_mut() {
            proposition.attrs = Some(PropositionAttributes {
              scope_id: branch_scope_id,
            });
            self.visit_proposition(proposition)?;
          }
        }

        if let Some((timeout, branch)) = data.after.as_mut() {
          timeout.attrs = Some(ExpressionAttributes {
            scope_id: attrs.scope_id,
          });
          self.visit_expression(timeout)?;

          let branch_scope_id = self.scope_arena.new_scope(attrs.scope_id);

          for proposition in branch.iter_mut() {
            proposition.attrs = Some(PropositionAttributes {
              scope_id: branch_scope_id,
            });
            self.visit_proposition(proposition)?;
          }
        }

        Ok(())
      }
    }
  }
}
