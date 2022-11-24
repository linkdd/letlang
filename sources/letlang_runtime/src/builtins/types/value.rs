use crate::core::{context::TaskContext, function::FunctionCoroutine, types::Type};
use crate::repr::Value;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct ValueType {
  pub llval: Value,
}

#[async_trait]
impl Type for ValueType {
  async fn to_string(&self, context: Arc<Mutex<TaskContext>>) -> String {
    format!("{}", self.llval.to_string(context.clone()).await)
  }

  async fn has(&self, _context: Arc<Mutex<TaskContext>>, _co: &FunctionCoroutine, llval: &Value) -> bool {
    match (&self.llval, llval) {
      (Value::Boolean(a), Value::Boolean(b)) => a == b,
      (Value::Number(a), Value::Number(b)) => a == b,
      (Value::String(a), Value::String(b)) => a == b,
      (Value::Atom(a), Value::Atom(b)) => a == b,
      _ => false,
    }
  }
}
