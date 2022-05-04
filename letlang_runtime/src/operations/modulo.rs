use crate::{Value, PrimitiveValue};

pub fn modulo(a: &Value, b: &Value) -> Result<Value, ()> {
  match (a, b) {
    (
      Value::Primitive(PrimitiveValue::Number(a)),
      Value::Primitive(PrimitiveValue::Number(b)),
    ) => {
      Ok(Value::Primitive(PrimitiveValue::Number(a.rem_euclid(*b))))
    },
    _ => {
      Err(())
    }
  }
}
