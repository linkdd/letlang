use crate::prelude::*;

use super::atom_interner::AtomInterner;
use super::scope::ScopeArena;

use letlang_ast::*;

use std::collections::HashMap;
use std::cell::RefCell;


pub struct Model<'compiler> {
  unit_registry: &'compiler HashMap<String, RefCell<Node<Unit>>>,
  atom_interner: &'compiler mut AtomInterner,
  scope_arena: &'compiler mut ScopeArena,
}


impl<'compiler> Model<'compiler> {
  pub fn new(
    unit_registry: &'compiler HashMap<String, RefCell<Node<Unit>>>,
    atom_interner: &'compiler mut AtomInterner,
    scope_arena: &'compiler mut ScopeArena,
  ) -> Self {
    Self { unit_registry, atom_interner, scope_arena }
  }

  pub fn visit(&mut self) -> Result<(), (String, CompilationError)> {
    let keys: Vec<String> = self.unit_registry.keys()
      .into_iter()
      .map(|key| key.clone())
      .collect();

    for unit_key in keys {
      self.visit_unit_key(unit_key.clone())
        .map_err(|err| (unit_key.clone(), err))?;
    }

    Ok(())
  }
}

mod unit;
mod statement;
mod proposition;
mod params;
mod typeref;
mod expression;
mod pattern;
mod literal;
mod symbol;
