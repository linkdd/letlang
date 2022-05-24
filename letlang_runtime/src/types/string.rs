use crate::{FunctionCoroutine, Context, Value, PrimitiveValue, Type};
use std::sync::{Arc, Mutex};
use async_trait::async_trait;

pub struct StringType;

#[async_trait]
impl Type for StringType {
  fn to_string(&self, _context: Arc<Mutex<Context>>) -> String {
    "string".to_string()
  }

  async fn has(&self, _co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    match llval {
      Value::Primitive(PrimitiveValue::String(_)) => true,
      _ => false,
    }
  }
}
