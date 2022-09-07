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
}
