use ast_core::*;
use ast_macros::*;

use llfront::{ast::*, SourceLocation};
use crate::{
  steps::scope::context::{ScopeBuilder, EnvRef, SymbolKind},
  prelude::*
};

model!{
  impl<'source> Transformer for ScopeBuilder {
    type InputData = Type<SourceLocation<'source>>;
    type InputMeta = SourceLocation<'source>;

    type OutputData = Type<(EnvRef, SourceLocation<'source>)>;
    type OutputMeta = (EnvRef, SourceLocation<'source>);

    type Error = CompilationError<'source>;

    visit {
      let env = self.env_stack.get_scope();

      let data = match node.get_data() {
        Type::Literal(lit_node) => Type::Literal(visit!(lit_node)),
        _ => todo!("type"),
      };

      Ok(Node::new(
        (env.clone(), node.get_meta().clone()),
        data,
      ))
    }
  }
}
