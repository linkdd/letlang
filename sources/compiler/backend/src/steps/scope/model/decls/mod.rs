use nonempty::NonEmpty;
use ast_core::*;
use ast_macros::*;

use llfront::{ast::*, SourceLocation};
use crate::{
  steps::scope::context::{ScopeBuilder, EnvRef, SymbolKind},
  prelude::*
};

model!{
  impl<'source> Transformer for ScopeBuilder {
    type InputData = NamedDeclaration<SourceLocation<'source>>;
    type InputMeta = SourceLocation<'source>;

    type OutputData = NamedDeclaration<(EnvRef, SourceLocation<'source>)>;
    type OutputMeta = (EnvRef, SourceLocation<'source>);

    type Error = CompilationError<'source>;

    visit {
      let env = self.env_stack.get_scope();

      let data = node.get_data();
      let sym_kind = match data.declaration.get_data() {
        Declaration::Class(_) => SymbolKind::Class {
          type_arity: data.type_params.len(),
          local_index: None,
        },
        Declaration::Effect(effect) => SymbolKind::Effect {
          type_arity: data.type_params.len(),
          call_arity: effect.params.len(),
        },
        Declaration::Function(func) => SymbolKind::Function {
          type_arity: data.type_params.len(),
          call_arity: func.params.len(),
        },
      };

      env.borrow_mut().insert(data.name.get_data().0.clone(), sym_kind);

      let public = data.public;
      let name = visit!(&data.name);
      let type_params = visit_many!(data.type_params);

      self.env_stack.enter_scope();
      let decl_scope = self.env_stack.get_scope();

      for (idx, type_param) in data.type_params.iter().enumerate() {
        decl_scope.borrow_mut().insert(
          type_param.get_data().0.clone(),
          SymbolKind::Class { type_arity: 0, local_index: Some(idx) },
        );
      }

      let declaration = visit!(&data.declaration);
      self.env_stack.leave_scope();

      Ok(Node::new(
        (env.clone(), node.get_meta().clone()),
        NamedDeclaration { public, name, type_params, declaration },
      ))
    }
  }
}


model!{
  impl<'source> Transformer for ScopeBuilder {
    type InputData = Declaration<SourceLocation<'source>>;
    type InputMeta = SourceLocation<'source>;

    type OutputData = Declaration<(EnvRef, SourceLocation<'source>)>;
    type OutputMeta = (EnvRef, SourceLocation<'source>);

    type Error = CompilationError<'source>;

    visit {
      let env = self.env_stack.get_scope();

      let data = match node.get_data() {
        Declaration::Class(_) => {
          todo!("decl class");
        },
        Declaration::Effect(_) => {
          todo!("decl effect");
        },
        Declaration::Function(func) => {
          let params = visit_many!(func.params);
          let return_type = visit!(&func.return_type);

          let func_arity = params.len();

          let mut clauses = vec![];

          for clause_node in func.clauses.iter() {
            self.env_stack.enter_scope();

            let clause_arity = clause_node.get_data().patterns.len();
            if clause_arity != func_arity {
              return Err(CompilationError {
                location: clause_node.get_meta().clone(),
                kind: CompilationErrorKind::ClauseArrity {
                  expected: func_arity,
                  got: clause_arity,
                },
              });
            }

            let clause = visit!(clause_node);
            self.env_stack.leave_scope();
            clauses.push(clause);
          }

          let clauses = NonEmpty::from_vec(clauses).unwrap();

          Declaration::Function(Function {
            tailrec: func.tailrec,
            params,
            return_type,
            clauses,
          })
        },
      };

      Ok(Node::new(
        (env.clone(), node.get_meta().clone()),
        data,
      ))
    }
  }
}
