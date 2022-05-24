use crate::{FunctionCoroutine, Context, Value, PrimitiveValue};
use std::sync::{Arc, Mutex};

pub async fn equal(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
  Ok(Value::Primitive(PrimitiveValue::Boolean(
    a == b
  )))
}
