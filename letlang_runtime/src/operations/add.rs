use crate::{FunctionCoroutine, Context, Value, PrimitiveValue};
use std::sync::{Arc, Mutex};

pub async fn add(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
  match (a, b) {
    (
      Value::Primitive(PrimitiveValue::Number(a)),
      Value::Primitive(PrimitiveValue::Number(b)),
    ) => {
      Ok(Value::Primitive(PrimitiveValue::Number(a + b)))
    },
    (
      Value::Primitive(PrimitiveValue::String(a)),
      Value::Primitive(PrimitiveValue::String(b)),
    ) => {
      Ok(Value::Primitive(PrimitiveValue::String(format!("{}{}", a, b))))
    },
    _ => {
      Err(())
    }
  }
}
