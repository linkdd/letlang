use std::collections::HashMap;
use std::cell::RefCell;

use super::{ScopeArena, SymbolKind};

pub struct Scope {
  symbols: RefCell<HashMap<String, (bool, SymbolKind)>>,
  parent_scope_id: Option<usize>,
}

impl Scope {
  pub fn new(parent_scope_id: Option<usize>) -> Self {
    Self { symbols: RefCell::new(HashMap::new()), parent_scope_id }
  }

  pub fn register_symbol(
    &self,
    key: String,
    public: bool,
    kind: SymbolKind,
  ) {
    let mut symbols = self.symbols.borrow_mut();
    symbols.insert(key, (public, kind));
  }

  pub fn lookup_symbol(
    &self,
    scope_arena: &ScopeArena,
    key: &String,
  ) -> Option<(bool, SymbolKind)> {
    let symbols = self.symbols.borrow();

    if symbols.contains_key(key) {
      symbols.get(key).map(|sym| sym.clone())
    }
    else if let Some(parent_scope_id) = self.parent_scope_id {
      scope_arena.get_scope(parent_scope_id).lookup_symbol(scope_arena, key)
    }
    else {
      None
    }
  }
}
