use crate::{Context, Value, PrimitiveValue};
use std::sync::{Arc, Mutex};

pub fn modulo(_context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
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
