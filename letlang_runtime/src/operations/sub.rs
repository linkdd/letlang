use crate::{Value, PrimitiveValue};

pub fn sub(a: &Value, b: &Value) -> Result<Value, ()> {
  match (a, b) {
    (
      Value::Primitive(PrimitiveValue::Number(a)),
      Value::Primitive(PrimitiveValue::Number(b)),
    ) => {
      Ok(Value::Primitive(PrimitiveValue::Number(a - b)))
    },
    _ => {
      Err(())
    }
  }
}
