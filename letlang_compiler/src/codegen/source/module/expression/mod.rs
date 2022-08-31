use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};



impl<'compiler> Generator<'compiler> {
  pub fn gen_expression(&self, node: &Node<Expression>) -> CompilationResult<String> {
    match node.data.as_ref() {
      Expression::Literal(lit) => {
        self.gen_literal(lit)
      },
      _ => {
        todo!();
      }
    }
  }
}

mod literal;
