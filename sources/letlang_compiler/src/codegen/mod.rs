use super::{
  atom_interner::AtomInterner,
  scope::ScopeArena,
  Toolchain,
  Target,
};

use letlang_ast::*;

use std::collections::HashMap;
use std::path::Path;
use std::error::Error;
use std::cell::RefCell;


pub struct Generator<'compiler> {
  unit_registry: &'compiler HashMap<String, RefCell<Node<Unit>>>,
  atom_interner: &'compiler mut AtomInterner,
  scope_arena: &'compiler ScopeArena,
  toolchain: &'compiler Toolchain,
  target: &'compiler Target,
}

impl<'compiler> Generator<'compiler> {
  pub fn new(
    unit_registry: &'compiler HashMap<String, RefCell<Node<Unit>>>,
    atom_interner: &'compiler mut AtomInterner,
    scope_arena: &'compiler ScopeArena,
    toolchain: &'compiler Toolchain,
    target: &'compiler Target,
  ) -> Self {
    Self { unit_registry, atom_interner, scope_arena, toolchain, target }
  }

  pub fn build<P: AsRef<Path>>(&mut self, target_dir: P) -> Result<(), Box<dyn Error>> {
    let mut workspace_members = vec!["executable".to_string()];

    for (unit_key, unit_node_cell) in self.unit_registry.iter() {
      let unit_node = unit_node_cell.borrow();
      self.gen_lib_crate(&target_dir, unit_key, &unit_node)?;
      workspace_members.push(format!("modules/{}", unit_key));
    }

    self.gen_exe_crate(&target_dir)?;
    self.gen_workspace(&target_dir, workspace_members)?;

    Ok(())
  }
}

mod cargo;
mod source;
