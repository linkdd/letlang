use super::env::{Environment, EnvRef};

pub struct EnvironmentStack {
  stack: Vec<EnvRef>,
}

impl EnvironmentStack {
  pub fn new() -> Self {
    Self { stack: vec![] }
  }

  pub fn enter_scope(&mut self) {
    let parent = self.stack.last().map(|e| e.clone());
    let new_scope = Environment::new(parent);
    self.stack.push(new_scope);
  }

  pub fn get_scope(&self) -> EnvRef {
    self.stack.last().map(|e| e.clone()).expect("expected at least one env")
  }

  pub fn leave_scope(&mut self) {
    self.stack.pop();
  }
}
