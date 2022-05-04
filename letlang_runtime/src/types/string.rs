use crate::{Value, PrimitiveValue, Type};

pub struct StringType;

impl Type for StringType {
  fn has(&self, llval: &Value) -> bool {
    match llval {
      Value::Primitive(PrimitiveValue::String(_)) => true,
      _ => false,
    }
  }
}
