use ast_core::*;
use ast_macros::*;

use llfront::{ast::*, SourceLocation};
use crate::{
  steps::scope::context::{ScopeBuilder, EnvRef, SymbolKind},
  prelude::*
};

mod import;
mod decls;
mod typ;
mod expr;
mod pat;

pub use self::{
  import::*,
  decls::*,
  typ::*,
  expr::*,
  pat::*,
};

model!{
  impl<'source> Transformer for ScopeBuilder {
    type InputData = Module<SourceLocation<'source>>;
    type InputMeta = SourceLocation<'source>;

    type OutputData = Module<(EnvRef, SourceLocation<'source>)>;
    type OutputMeta = (EnvRef, SourceLocation<'source>);

    type Error = CompilationError<'source>;

    visit {
      let root_env = self.env_stack.get_scope();

      let mut mod_env = root_env;
      for path_segment in node.get_data().path.iter() {
        self.env_stack.enter_scope();
        let new_env = self.env_stack.get_scope();

        mod_env.borrow_mut().insert(
          path_segment.get_data().0.clone(),
          SymbolKind::Module { env: new_env.clone() },
        );

        mod_env = new_env;
      }

      let path = visit_many!(node.get_data().path).try_into().unwrap();
      let imports = visit_many!(node.get_data().imports);
      let declarations = visit_many!(node.get_data().declarations);

      for _ in node.get_data().path.iter() {
        self.env_stack.leave_scope();
      }

      Ok(Node::new(
        (mod_env.clone(), node.get_meta().clone()),
        Module { path, imports, declarations },
      ))
    }
  }
}

model!{
  impl<'source> Transformer for ScopeBuilder {
    type InputData = Identifier;
    type InputMeta = SourceLocation<'source>;

    type OutputData = Identifier;
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
