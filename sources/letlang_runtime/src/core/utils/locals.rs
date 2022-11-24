use crate::core::constraint::Constraint;
use crate::repr::Value;

use std::collections::HashMap;


pub struct Locals<'scope> {
  variables: HashMap<String, Value>,
  constraints: Vec<Box<dyn Constraint>>,
  parent_scope: Option<&'scope Locals<'scope>>
}

impl<'scope> Locals<'scope> {
  pub fn new(parent_scope: Option<&'scope Locals>) -> Self {
    Self {
      variables: HashMap::new(),
      constraints: vec![],
      parent_scope,
    }
  }

  pub fn register_symbol(&mut self, symbol_name: &str, symbol_val: Value) {
    self.variables.insert(symbol_name.to_string(), symbol_val);
  }

  pub fn register_constraint(&mut self, constraint: Box<dyn Constraint>) {
    self.constraints.push(constraint);
  }

  pub fn lookup_symbol(&self, symbol_name: &str) -> Option<&Value> {
    match self.variables.get(symbol_name) {
      None => {
        match self.parent_scope {
          None => None,
          Some(scope) => {
            scope.lookup_symbol(symbol_name)
          }
        }
      },
      Some(val) => {
        Some(val)
      }
    }
  }

  pub fn iter_constraints(&self) -> Box<dyn Iterator<Item = &Box<dyn Constraint>> + Send + '_> {
    match self.parent_scope {
      None => {
        Box::new(self.constraints.iter())
      },
      Some(scope) => {
        Box::new(scope.iter_constraints().chain(self.constraints.iter()))
      }
    }
  }
}
