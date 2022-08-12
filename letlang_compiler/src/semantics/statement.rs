use crate::prelude::*;
use super::Model;

use letlang_parser::ast::{
  Node,
  Statement,
  statement::{
    ImportStatement,
    EffectDeclStatement,
    ClassDeclStatement,
    FuncDeclStatement,
  }
};


impl Model {
  pub fn visit_statement(&mut self, node: &Node<Statement>) -> CompilationResult<()> {
    let result = match node.data.as_ref() {
      Statement::Import(data) => {
        self.visit_statement_import(data)
      },
      Statement::EffectDecl(data) => {
        self.visit_statement_effect(data)
      },
      Statement::ClassDecl(data) => {
        self.visit_statement_class(data)
      },
      Statement::FuncDecl(data) => {
        self.visit_statement_func(data)
      },
    };

    result.map_err(|err| {
      if !err.is_located() {
        CompilationError::new_located(&node.location, err.message)
      }
      else {
        err
      }
    })
  }

  fn visit_statement_import(&mut self, data: &ImportStatement) -> CompilationResult<()> {
    match (data.path.last(), &data.alias) {
      (None, _) => {
        Err(CompilationError::new("cannot import empty path".to_string()))
      },
      (Some(sym), None) => {
        todo!()
      },
      (_, Some(alias)) => {
        todo!()
      },
    }
  }

  fn visit_statement_effect(&mut self, data: &EffectDeclStatement) -> CompilationResult<()> {
    todo!()
  }

  fn visit_statement_class(&mut self, data: &ClassDeclStatement) -> CompilationResult<()> {
    for proposition in data.constraints.iter() {
      self.visit_proposition(proposition)?;
    }

    todo!();
  }

  fn visit_statement_func(&mut self, data: &FuncDeclStatement) -> CompilationResult<()> {
    for proposition in data.body.iter() {
      self.visit_proposition(proposition)?;
    }

    todo!();
  }
}
