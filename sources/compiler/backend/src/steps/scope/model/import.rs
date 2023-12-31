use ast_core::*;
use ast_macros::*;

use llfront::{ast::*, SourceLocation};
use crate::{
  steps::scope::context::{ScopeBuilder, EnvRef},
  prelude::*
};

model!{
  impl<'source> Transformer for ScopeBuilder {
    type InputData = Import<SourceLocation<'source>>;
    type InputMeta = SourceLocation<'source>;

    type OutputData = Import<(EnvRef, SourceLocation<'source>)>;
    type OutputMeta = (EnvRef, SourceLocation<'source>);

    type Error = CompilationError<'source>;

    visit {
      match node.get_data() {
        Import::Symbol { path, symbols } => todo!("import symbols"),
        Import::Module { path, alias } => todo!("import alias"),
      }
    }
  }
}
