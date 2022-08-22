use crate::prelude::*;
use letlang_parser::{ast, ast::Node};
use super::CodeGenerator;

impl<'gen, 'ctx> CodeGenerator<'gen, 'ctx> {
  pub fn gen_statement(&self, node: &Node<ast::Statement>) -> CompilationResult<String> {
    match node.data.as_ref() {
      ast::Statement::Import(data) => {
        self.gen_statement_import(data)
      },
      ast::Statement::EffectDecl(_) => todo!(),
      ast::Statement::ClassDecl(_) => todo!(),
      ast::Statement::FuncDecl(_) => todo!(),
    }
  }

  fn gen_statement_import(&self, data: &ast::statement::ImportStatement) -> CompilationResult<String> {
    match data.alias.as_ref() {
      None => {
        Ok(format!("pub(crate) use lldep_{};", data.path.join("_")))
      },
      Some(alias) => {
        Ok(format!("pub(crate) use lldep_{} as lldep_{}", data.path.join("_"), alias))
      }
    }
  }

  fn gen_statement_func(&self, data: &ast::statement::FuncDeclStatement) -> CompilationResult<String> {
    todo!();
  }
}
