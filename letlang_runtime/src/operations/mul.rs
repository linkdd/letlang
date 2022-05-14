use crate::{Context, Value, PrimitiveValue, Type, types};
use std::sync::{Arc, Mutex};

pub fn mul(context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
  let t_int = types::IntegerType {};

  match (a, b) {
    (
      Value::Primitive(PrimitiveValue::Number(a)),
      Value::Primitive(PrimitiveValue::Number(b)),
    ) => {
      Ok(Value::Primitive(PrimitiveValue::Number(a * b)))
    },
    (
      Value::Primitive(PrimitiveValue::String(a)),
      Value::Primitive(PrimitiveValue::Number(n)),
    ) => {

      if t_int.has(context, b) {
        let mut res = String::new();
        let mut count = 0;

        while &(count as f64) < n {
          res = format!("{}{}", res, a);
          count += 1;
        }

        Ok(Value::Primitive(PrimitiveValue::String(res)))
      }
      else {
        Err(())
      }
    }
    _ => {
      Err(())
    }
  }
}
