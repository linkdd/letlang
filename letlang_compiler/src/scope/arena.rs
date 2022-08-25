use super::Scope;

pub struct ScopeArena {
  scopes: Vec<Scope>,
  next_scope_id: usize,
}

impl ScopeArena {
  pub fn new() -> Self {
    Self { scopes: vec![], next_scope_id: 0 }
  }

  fn next_id(&mut self) -> usize {
    let scope_id = self.next_scope_id;
    self.next_scope_id += 1;
    scope_id
  }

  pub fn new_root_scope(&mut self) -> usize {
    let scope = Scope::new(None);
    self.scopes.push(scope);
    self.next_id()
  }

  pub fn new_scope(&mut self, parent_scope_id: usize) -> usize {
    let scope = Scope::new(Some(parent_scope_id));
    self.scopes.push(scope);
    self.next_id()
  }

  pub fn get_scope(&self, id: usize) -> &Scope {
    &self.scopes[id]
  }
}
