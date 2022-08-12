use crate::prelude::*;

use string_interner::{
  StringInterner,
  backend::StringBackend,
  symbol::SymbolUsize,
};

use letlang_parser::{ast, ast::Node};


pub enum SymbolType {
  Module,
  Effect,
  Class,
  Function,
  Variable
}


pub struct Model {
  interner: StringInterner<StringBackend<SymbolUsize>>,
}

impl Model {
  pub fn new() -> Self {
    Self { interner: StringInterner::new() }
  }

  pub fn visit(&mut self, units: &Vec<Node<ast::Unit>>) -> CompilationResult<()> {
    for unit in units.iter() {
      self.visit_unit(unit)?;
    }

    Ok(())
  }

  pub fn visit_unit(&mut self, node: &Node<ast::Unit>) -> CompilationResult<()> {
    for statement in node.data.statements.iter() {
      self.visit_statement(statement)?;
    }

    Ok(())
  }
}

mod statement;
mod proposition;
mod expression;
mod types;
