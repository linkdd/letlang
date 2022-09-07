use std::collections::HashMap;

use crate::repr::Atom;


pub struct AtomTable {
  index: HashMap<Atom, &'static str>,
}

impl AtomTable {
  pub fn new() -> Self {
    Self { index: HashMap::new() }
  }

  pub fn register(&mut self, atom: Atom, repr: &'static str) {
    self.index.insert(atom, repr);
  }

  pub fn lookup(&self, atom: &Atom) -> &'static str {
    self.index.get(atom).expect("unknown atom")
  }

  pub fn from_repr(&self, repr: &'static str) -> Atom {
    for (atom, atom_repr) in self.index.iter() {
      if &repr == atom_repr {
        return atom.clone();
      }
    }

    panic!("no atom {repr} found");
  }
}
