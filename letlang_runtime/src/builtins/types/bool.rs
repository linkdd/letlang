use crate::core::{context::TaskContext, function::FunctionCoroutine, type_trait::Type};
use crate::repr::Value;

use async_trait::async_trait;

#[derive(Debug)]
pub struct BooleanType;

#[async_trait]
impl Type for BooleanType {
  fn to_string(&self, _context: &mut TaskContext) -> String {
    "bool".to_string()
  }

  async fn has(&self, _context: &mut TaskContext, _co: &FunctionCoroutine, llval: &Value) -> bool {
    match llval {
      Value::Boolean(_) => true,
      _ => false,
    }
  }
}
