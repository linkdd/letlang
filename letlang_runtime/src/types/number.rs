use crate::{Context, Value, PrimitiveValue, Type};

pub struct NumberType;

impl Type for NumberType {
  fn to_string(&self, _context: Arc<Mutex<Context>>) -> String {
    "number".to_string()
  }

  fn has(&self, _context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    match llval {
      Value::Primitive(PrimitiveValue::Number(_)) => true,
      _ => false,
    }
  }
}

pub struct IntegerType;
use std::sync::{Arc, Mutex};

impl Type for IntegerType {
  fn to_string(&self, _context: Arc<Mutex<Context>>) -> String {
    "int".to_string()
  }

  fn has(&self, _context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    match llval {
      Value::Primitive(PrimitiveValue::Number(a)) => a.fract() == 0.0,
      _ => false,
    }
  }
}
