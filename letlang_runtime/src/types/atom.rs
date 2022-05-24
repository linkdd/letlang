use crate::{FunctionCoroutine, Context, Value, PrimitiveValue, Type};
use std::sync::{Arc, Mutex};
use async_trait::async_trait;

pub struct AtomType;

#[async_trait]
impl Type for AtomType {
  fn to_string(&self, _context: Arc<Mutex<Context>>) -> String {
    "atom".to_string()
  }

  async fn has(&self, _co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    match llval {
      Value::Primitive(PrimitiveValue::Atom(_)) => true,
      _ => false,
    }
  }
}
