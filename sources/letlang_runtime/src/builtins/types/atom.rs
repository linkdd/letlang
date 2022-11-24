use crate::core::{context::TaskContext, function::FunctionCoroutine, types::Type};
use crate::repr::Value;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;


#[derive(Debug)]
pub struct AtomType;

#[async_trait]
impl Type for AtomType {
  async fn to_string(&self, _context: Arc<Mutex<TaskContext>>) -> String {
    "atom".to_string()
  }

  async fn has(&self, _context: Arc<Mutex<TaskContext>>, _co: &FunctionCoroutine, llval: &Value) -> bool {
    match llval {
      Value::Atom(_) => true,
      _ => false,
    }
  }
}
