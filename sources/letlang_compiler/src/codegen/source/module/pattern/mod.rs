use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};

impl<'compiler> Generator<'compiler> {
  pub fn gen_pattern(
    &self,
    node: &Node<Pattern>,
  ) -> CompilationResult<String> {
    match node.data.as_ref() {
      Pattern::Symbol(sym) => {
        self.gen_pattern_symbol(sym)
      },
      Pattern::Literal(lit) => {
        self.gen_pattern_value(lit)
      },
      Pattern::Tuple(data) => {
        self.gen_pattern_tuple(&node.location, data)
      },
      _ => {
        todo!();
      }
    }
  }
}

mod symbol;
mod value;
mod tuple;
