use ast_core::*;
use ast_macros::*;

use llfront::{ast::*, SourceLocation};
use crate::{
  steps::scope::context::{ScopeBuilder, EnvRef},
  prelude::*
};

model!{
  impl<'source> Transformer for ScopeBuilder {
    type InputData = Literal;
    type InputMeta = SourceLocation<'source>;

    type OutputData = Literal;
    type OutputMeta = (EnvRef, SourceLocation<'source>);

    type Error = CompilationError<'source>;

    visit {
      let env = self.env_stack.get_scope();

      Ok(Node::new(
        (env.clone(), node.get_meta().clone()),
        node.get_data().clone(),
      ))
    }
  }
}
