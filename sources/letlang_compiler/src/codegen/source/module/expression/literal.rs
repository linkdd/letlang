use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};

use string_interner::Symbol;


impl<'compiler> Generator<'compiler> {
  pub fn gen_literal(&self, node: &Node<Literal>) -> CompilationResult<String> {
    match node.data.as_ref() {
      Literal::Boolean(val) => {
        Ok(format!("Value::Boolean({})", val))
      },
      Literal::Number(val) => {
        Ok(format!("Value::Number({})", val))
      },
      Literal::String(val) => {
        Ok(format!("Value::String({:?})", val))
      },
      Literal::Atom(Atom(repr)) => {
        let sym = self.atom_interner.get(repr).unwrap().to_usize();
        Ok(format!("Value::Atom(Atom({}))", sym))
      }
    }
  }
}
