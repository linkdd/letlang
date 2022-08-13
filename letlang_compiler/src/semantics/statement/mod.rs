use crate::prelude::*;
pub use super::{Model, Visitor};

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


impl<V: Visitor> Model<V> {
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

    Self::locate_error(result, &node.location)
  }

  fn visit_statement_import(&mut self, data: &ImportStatement) -> CompilationResult<()> {
    self.call_visitor(data)
  }

  fn visit_statement_effect(&mut self, data: &EffectDeclStatement) -> CompilationResult<()> {
    self.call_visitor(data)?;

    for call_param in data.call_params.iter() {
      self.visit_call_param(call_param)?;
    }

    self.visit_typeref(&data.return_type)?;

    Ok(())
  }

  fn visit_statement_class(&mut self, data: &ClassDeclStatement) -> CompilationResult<()> {
    self.call_visitor(data)?;

    for type_param in data.type_params.iter() {
      self.visit_type_param(type_param)?;
    }

    self.visit_cons_param(&data.cons_param)?;

    for proposition in data.constraints.iter() {
      self.visit_proposition(proposition)?;
    }

    Ok(())
  }

  fn visit_statement_func(&mut self, data: &FuncDeclStatement) -> CompilationResult<()> {
    self.call_visitor(data)?;

    for type_param in data.type_params.iter() {
      self.visit_type_param(type_param)?;
    }

    for call_param in data.call_params.iter() {
      self.visit_call_param(call_param)?;
    }

    self.visit_typeref(&data.return_type)?;

    for proposition in data.body.iter() {
      self.visit_proposition(proposition)?;
    }

    Ok(())
  }
}

mod proposition;
