use crate::{Value, PrimitiveValue, Type};

pub struct BooleanType;

impl Type for BooleanType {
  fn has(&self, llval: &Value) -> bool {
    match llval {
      Value::Primitive(PrimitiveValue::Boolean(_)) => true,
      _ => false,
    }
  }
}
