use crate::prelude::*;
pub use super::Model;

use letlang_parser::ast::{
  Node,
  types::TypeRef,
};


impl Model {
  pub fn visit_typeref(&mut self, node: &Node<TypeRef>) -> CompilationResult<()> {
    todo!();
  }
}
