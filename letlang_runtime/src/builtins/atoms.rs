use crate::repr::Atom;
use std::collections::HashMap;

pub struct BuiltinAtoms {
  atoms: HashMap<String, Atom>,
}

impl BuiltinAtoms {
  pub fn new() -> Self {
    Self { atoms: HashMap::new() }
  }

  pub fn add(&mut self, name: &str, atom: Atom) {
    self.atoms.insert(name.to_string(), atom);
  }

  pub fn get(&self, name: &str) -> Atom {
    let owned_name = name.to_string();
    self.atoms.get(&owned_name).expect("unknown builtin atom").clone()
  }
}
