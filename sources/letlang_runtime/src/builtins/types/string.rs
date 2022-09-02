use crate::core::{context::TaskContext, function::FunctionCoroutine, type_trait::Type};
use crate::repr::Value;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;


#[derive(Debug)]
pub struct StringType;

#[async_trait]
impl Type for StringType {
  async fn to_string(&self, _context: Arc<Mutex<TaskContext>>) -> String {
    "string".to_string()
  }

  async fn has(&self, _context: Arc<Mutex<TaskContext>>, _co: &FunctionCoroutine, llval: &Value) -> bool {
    match llval {
      Value::String(_) => true,
      _ => false,
    }
  }
}
