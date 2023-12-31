use ast_core::*;
use ast_macros::*;

use llfront::{ast::*, SourceLocation};
use crate::{
  steps::scope::context::{ScopeBuilder, EnvRef, SymbolKind},
  prelude::*
};

model!{
  impl<'source> Transformer for ScopeBuilder {
    type InputData = Clause<SourceLocation<'source>>;
    type InputMeta = SourceLocation<'source>;

    type OutputData = Clause<(EnvRef, SourceLocation<'source>)>;
    type OutputMeta = (EnvRef, SourceLocation<'source>);

    type Error = CompilationError<'source>;

    visit {
      let env = self.env_stack.get_scope();

      let data = node.get_data();

      let patterns = visit_many!(data.patterns);
      let guard = match &data.guard {
        Some(expr) => Some(visit!(expr)),
        None => None,
      };
      let body = visit_many!(data.body).try_into().unwrap();

      Ok(Node::new(
        (env.clone(), node.get_meta().clone()),
        Clause { patterns, guard, body },
      ))
    }
  }
}


model!{
  impl<'source> Transformer for ScopeBuilder {
    type InputData = Pattern<SourceLocation<'source>>;
    type InputMeta = SourceLocation<'source>;

    type OutputData = Pattern<(EnvRef, SourceLocation<'source>)>;
    type OutputMeta = (EnvRef, SourceLocation<'source>);

    type Error = CompilationError<'source>;

    visit {
      todo!("pattern")
    }
  }
}
