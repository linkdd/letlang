use crate::{Value, PrimitiveValue, Type};

pub struct AtomType;

impl Type for AtomType {
  fn has(&self, llval: &Value) -> bool {
    match llval {
      Value::Primitive(PrimitiveValue::Atom(_)) => true,
      _ => false,
    }
  }
}
