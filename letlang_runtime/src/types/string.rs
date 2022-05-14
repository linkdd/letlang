use crate::{Context, Value, PrimitiveValue, Type};
use std::sync::{Arc, Mutex};

pub struct StringType;

impl Type for StringType {
  fn to_string(&self, _context: Arc<Mutex<Context>>) -> String {
    "string".to_string()
  }

  fn has(&self, _context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    match llval {
      Value::Primitive(PrimitiveValue::String(_)) => true,
      _ => false,
    }
  }
}
