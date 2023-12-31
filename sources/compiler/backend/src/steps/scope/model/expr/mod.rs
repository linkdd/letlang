use ast_core::*;
use ast_macros::*;

use llfront::{ast::*, SourceLocation};
use crate::{
  steps::scope::context::{ScopeBuilder, EnvRef, SymbolKind},
  prelude::*
};

mod lit;

pub use self::{
  lit::*,
};

model!{
  impl<'source> Transformer for ScopeBuilder {
    type InputData = Expression<SourceLocation<'source>>;
    type InputMeta = SourceLocation<'source>;

    type OutputData = Expression<(EnvRef, SourceLocation<'source>)>;
    type OutputMeta = (EnvRef, SourceLocation<'source>);

    type Error = CompilationError<'source>;

    visit {
      let env = self.env_stack.get_scope();

      let data = match node.get_data() {
        Expression::Literal(lit_node) => Expression::Literal(visit!(lit_node)),
        _ => todo!("expr"),
      };

      Ok(Node::new(
        (env.clone(), node.get_meta().clone()),
        data,
      ))
    }
  }
}
