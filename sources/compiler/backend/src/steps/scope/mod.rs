use ast_core::*;
use llfront::{ast::*, SourceLocation};

use crate::prelude::*;

mod context;
mod model;

use self::context::ScopeBuilder;
pub use self::context::{EnvRef, SymbolKind};

pub fn transform<'source>(
  /* bmi */
  ast: &AST<SourceLocation<'source>>
) -> Result<'source, (EnvRef, AST<(EnvRef, SourceLocation<'source>)>)> {
  let mut context = ScopeBuilder::new(/* bmi */);
  let root_env = context.env_stack.get_scope();

  let node = context.visit(&ast.0, ())?;

  Ok((root_env.clone(), AST(node)))
}
