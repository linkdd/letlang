use crate::{Value, PrimitiveValue, Type};

pub struct NumberType;

impl Type for NumberType {
  fn has(&self, llval: &Value) -> bool {
    match llval {
      Value::Primitive(PrimitiveValue::Number(_)) => true,
      _ => false,
    }
  }
}

pub struct IntegerType;

impl Type for IntegerType {
  fn has(&self, llval: &Value) -> bool {
    match llval {
      Value::Primitive(PrimitiveValue::Number(a)) => a.fract() == 0.0,
      _ => false,
    }
  }
}
