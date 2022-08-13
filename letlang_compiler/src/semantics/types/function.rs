use crate::prelude::*;
pub use super::{Model, Visitor};

use letlang_parser::ast::types::*;

impl<V: Visitor> Model<V> {
  pub fn visit_typeref_signature(&mut self, signature: &FunctionSignature) -> CompilationResult<()> {
    self.call_visitor(signature)?;

    for param in signature.params.iter() {
      self.visit_typeref(param)?;
    }

    self.visit_typeref(&signature.return_type)?;

    Ok(())
  }
}
