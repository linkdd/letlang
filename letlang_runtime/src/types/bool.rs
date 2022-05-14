use crate::{Context, Value, PrimitiveValue, Type};
use std::sync::{Arc, Mutex};

pub struct BooleanType;

impl Type for BooleanType {
  fn to_string(&self, _context: Arc<Mutex<Context>>) -> String {
    "bool".to_string()
  }

  fn has(&self, _context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    match llval {
      Value::Primitive(PrimitiveValue::Boolean(_)) => true,
      _ => false,
    }
  }
}
