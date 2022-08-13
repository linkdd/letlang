use crate::prelude::*;


use letlang_parser::{ast, ast::{LocationInfo, Node}};


pub trait Visitor {
  fn match_node(&self, node: &dyn std::any::Any) -> bool;
  fn visit_node(&self, node: &dyn std::any::Any) -> CompilationResult<()>;
}


pub struct Model<V: Visitor> {
  visitor: V,
}

impl<V: Visitor> Model<V> {
  pub fn new(visitor: V) -> Self {
    Self { visitor }
  }

  pub fn locate_error(
    result: CompilationResult<()>,
    location: &LocationInfo,
  ) -> CompilationResult<()> {
    result.map_err(|err| {
      if !err.is_located() {
        CompilationError::new_located(location, err.message)
      }
      else {
        err
      }
    })
  }

  pub fn call_visitor(&mut self, data: &dyn std::any::Any) -> CompilationResult<()> {
    if self.visitor.match_node(data) {
      self.visitor.visit_node(data)?;
    }

    Ok(())
  }

  pub fn visit(&mut self, units: &Vec<Node<ast::Unit>>) -> CompilationResult<()> {
    for unit in units.iter() {
      self.visit_unit(unit)?;
    }

    Ok(())
  }

  pub fn visit_unit(&mut self, node: &Node<ast::Unit>) -> CompilationResult<()> {
    Self::locate_error(
      self.call_visitor(node.data.as_ref()),
      &node.location
    )?;

    for statement in node.data.statements.iter() {
      self.visit_statement(statement)?;
    }

    Ok(())
  }
}

mod statement;
mod params;
mod expression;
mod types;
