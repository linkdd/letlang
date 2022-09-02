use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  statement::*,
};


impl<'compiler> Generator<'compiler> {
  pub fn gen_proposition(&self, node: &Node<Proposition>) -> CompilationResult<String> {
    match node.data.as_ref() {
      Proposition::Constraint(constraint) => {
        todo!();
      },
      Proposition::Evaluation(expr) => {
        self.gen_expression(expr)
      }
    }
  }
}
