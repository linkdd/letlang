use crate::prelude::*;

use super::Model;

use letlang_ast::{
  *,
  expression::*,
};

impl<'compiler> Model<'compiler> {
  pub fn visit_literal(&mut self, node: &mut Node<Literal>) -> CompilationResult<()> {
    match node.data.as_ref() {
      Literal::Atom(atom) => {
        let Atom(repr) = atom;
        self.atom_interner.get_or_intern(repr);
      },
      _ => {},
    }

    Ok(())
  }
}
