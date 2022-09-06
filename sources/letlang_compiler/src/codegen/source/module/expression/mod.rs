use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};


impl<'compiler> Generator<'compiler> {
  pub fn gen_expression(&self, node: &Node<Expression>) -> CompilationResult<String> {
    let attrs = node.attrs.as_ref().unwrap();

    match node.data.as_ref() {
      Expression::Literal(lit) => {
        self.gen_literal(lit)
      },
      Expression::FunctionCall(data) => {
        self.gen_function_call(&node.location, data)
      },
      Expression::EffectCall(data) => {
        self.gen_effect_call(&node.location, data)
      },
      Expression::Symbol(sym) => {
        self.gen_symbol(&node.location, sym, attrs.scope_id)
      },
      _ => {
        todo!();
      }
    }
  }
}

mod literal;
mod function;
mod effect;
mod symbol;
