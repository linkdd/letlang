use std::{
  collections::HashMap,
  rc::Rc,
  cell::RefCell,
};

use super::symbol::SymbolKind;

pub type EnvRef = Rc<RefCell<Environment>>;

pub struct Environment {
  locals: HashMap<String, SymbolKind>,
  parent: Option<EnvRef>,
}

impl Environment {
  pub fn new(parent: Option<EnvRef>) -> EnvRef {
    Rc::new(RefCell::new(Self {
      locals: HashMap::new(),
      parent,
    }))
  }

  pub fn lookup(&self, symbol_name: String) -> Option<SymbolKind> {
    match (self.locals.get(&symbol_name), &self.parent) {
      (Some(symbol_kind), _) => Some(symbol_kind.clone()),
      (None, Some(env)) => env.borrow().lookup(symbol_name),
      (None, None) => None,
    }
  }

  pub fn get(&self, symbol_name: String) -> Option<SymbolKind> {
    self.locals.get(&symbol_name).map(|sym| sym.clone())
  }

  pub fn insert(&mut self, symbol_name: String, symbol_kind: SymbolKind) {
    self.locals.insert(symbol_name, symbol_kind);
  }
}
