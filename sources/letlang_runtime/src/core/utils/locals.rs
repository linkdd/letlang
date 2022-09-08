use crate::repr::Value;

use std::collections::HashMap;


pub struct Locals<'scope> {
  variables: HashMap<&'static str, Value>,
  parent_scope: Option<&'scope Locals<'scope>>
}

impl<'scope> Locals<'scope> {
  pub fn new(parent_scope: Option<&'scope Locals>) -> Self {
    Self {
      variables: HashMap::new(),
      parent_scope,
    }
  }

  pub fn register_symbol(&mut self, symbol_name: &'static str, symbol_val: Value) {
    self.variables.insert(symbol_name, symbol_val);
  }

  pub fn lookup_symbol(&self, symbol_name: &'static str) -> Option<&Value> {
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
}
