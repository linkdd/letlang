use crate::{FunctionCoroutine, Context, Value, PrimitiveValue, types, Type};
use std::sync::{Arc, Mutex};

pub async fn neg(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value) -> Result<Value, ()> {
  match a {
    Value::Primitive(PrimitiveValue::Number(a)) => {
      Ok(Value::Primitive(PrimitiveValue::Number(-a)))
    },
    _ => {
      Err(())
    }
  }
}

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

pub async fn sub(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
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

pub async fn mul(co: &FunctionCoroutine, context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
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

      if t_int.has(co, context, b).await {
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

pub async fn div(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
  match (a, b) {
    (
      Value::Primitive(PrimitiveValue::Number(a)),
      Value::Primitive(PrimitiveValue::Number(b)),
    ) => {
      Ok(Value::Primitive(PrimitiveValue::Number(a / b)))
    },
    _ => {
      Err(())
    }
  }
}

pub async fn modulo(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
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
