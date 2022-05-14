use crate::{Context, Value, PrimitiveValue, Type};
use std::sync::{Arc, Mutex};

pub struct AtomType;

impl Type for AtomType {
  fn to_string(&self, _context: Arc<Mutex<Context>>) -> String {
    "atom".to_string()
  }

  fn has(&self, _context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    match llval {
      Value::Primitive(PrimitiveValue::Atom(_)) => true,
      _ => false,
    }
  }
}
