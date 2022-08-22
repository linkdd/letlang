use crate::core::{TaskContext, function::FunctionCoroutine, type_trait::Type};
use crate::repr::Value;

use async_trait::async_trait;

#[derive(Debug)]
pub struct ValueType {
  pub llval: Value,
}

#[async_trait]
impl Type for ValueType {
  fn to_string(&self, _context: &mut TaskContext) -> String {
    format!("{}", self.llval)
  }

  async fn has(&self, _context: &mut TaskContext, _co: &FunctionCoroutine, llval: &Value) -> bool {
    match (&self.llval, llval) {
      (Value::Boolean(a), Value::Boolean(b)) => a == b,
      (Value::Number(a), Value::Number(b)) => a == b,
      (Value::String(a), Value::String(b)) => a == b,
      (Value::Atom(a), Value::Atom(b)) => a == b,
      _ => false,
    }
  }
}
