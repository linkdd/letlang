use crate::{FunctionCoroutine, Context, Value, PrimitiveValue, Type};
use std::sync::{Arc, Mutex};
use async_trait::async_trait;

pub struct BooleanType;

#[async_trait]
impl Type for BooleanType {
  fn to_string(&self, _context: Arc<Mutex<Context>>) -> String {
    "bool".to_string()
  }

  async fn has(&self, _co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    match llval {
      Value::Primitive(PrimitiveValue::Boolean(_)) => true,
      _ => false,
    }
  }
}
