use crate::{FunctionCoroutine, Context, Value, PrimitiveValue, Type};
use async_trait::async_trait;

pub struct NumberType;

#[async_trait]
impl Type for NumberType {
  fn to_string(&self, _context: Arc<Mutex<Context>>) -> String {
    "number".to_string()
  }

  async fn has(&self, _co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    match llval {
      Value::Primitive(PrimitiveValue::Number(_)) => true,
      _ => false,
    }
  }
}

pub struct IntegerType;
use std::sync::{Arc, Mutex};

#[async_trait]
impl Type for IntegerType {
  fn to_string(&self, _context: Arc<Mutex<Context>>) -> String {
    "int".to_string()
  }

  async fn has(&self, _co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    match llval {
      Value::Primitive(PrimitiveValue::Number(a)) => a.fract() == 0.0,
      _ => false,
    }
  }
}
