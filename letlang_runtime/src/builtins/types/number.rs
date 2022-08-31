use crate::core::{context::TaskContext, function::FunctionCoroutine, type_trait::Type};
use crate::repr::Value;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;


#[derive(Debug)]
pub struct NumberType;

#[async_trait]
impl Type for NumberType {
  async fn to_string(&self, _context: Arc<Mutex<TaskContext>>) -> String {
    "number".to_string()
  }

  async fn has(&self, _context: Arc<Mutex<TaskContext>>, _co: &FunctionCoroutine, llval: &Value) -> bool {
    match llval {
      Value::Number(_) => true,
      _ => false,
    }
  }
}

#[derive(Debug)]
pub struct IntegerType;

#[async_trait]
impl Type for IntegerType {
  async fn to_string(&self, _context: Arc<Mutex<TaskContext>>) -> String {
    "int".to_string()
  }

  async fn has(&self, _context: Arc<Mutex<TaskContext>>, _co: &FunctionCoroutine, llval: &Value) -> bool {
    match llval {
      Value::Number(n) => n.fract() == 0.0,
      _ => false,
    }
  }
}
