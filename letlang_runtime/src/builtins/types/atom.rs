use crate::core::{TaskContext, function::FunctionCoroutine, type_trait::Type};
use crate::repr::Value;

use async_trait::async_trait;

#[derive(Debug)]
pub struct AtomType;

#[async_trait]
impl Type for AtomType {
  fn to_string(&self, _context: &mut TaskContext) -> String {
    "atom".to_string()
  }

  async fn has(&self, _context: &mut TaskContext, _co: &FunctionCoroutine, llval: &Value) -> bool {
    match llval {
      Value::Atom(_) => true,
      _ => false,
    }
  }
}
