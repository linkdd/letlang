use crate::{FunctionCoroutine, Context, Value, PrimitiveValue};
use std::sync::{Arc, Mutex};

pub async fn not(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value) -> Result<Value, ()> {
  match a {
    Value::Primitive(PrimitiveValue::Boolean(a)) => {
      Ok(Value::Primitive(PrimitiveValue::Boolean(!*a)))
    },
    _ => {
      Err(())
    }
  }
}

pub async fn and(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
  match (a, b) {
    (
      Value::Primitive(PrimitiveValue::Boolean(a)),
      Value::Primitive(PrimitiveValue::Boolean(b)),
    ) => {
      Ok(Value::Primitive(PrimitiveValue::Boolean(*a && *b)))
    },
    _ => {
      Err(())
    }
  }
}

pub async fn or(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
  match (a, b) {
    (
      Value::Primitive(PrimitiveValue::Boolean(a)),
      Value::Primitive(PrimitiveValue::Boolean(b)),
    ) => {
      Ok(Value::Primitive(PrimitiveValue::Boolean(*a || *b)))
    },
    _ => {
      Err(())
    }
  }
}

pub async fn imply(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
  match (a, b) {
    (
      Value::Primitive(PrimitiveValue::Boolean(a)),
      Value::Primitive(PrimitiveValue::Boolean(b)),
    ) => {
      Ok(Value::Primitive(PrimitiveValue::Boolean(!*a || (*a && *b))))
    },
    _ => {
      Err(())
    }
  }
}

pub async fn bicond(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
  match (a, b) {
    (
      Value::Primitive(PrimitiveValue::Boolean(a)),
      Value::Primitive(PrimitiveValue::Boolean(b)),
    ) => {
      Ok(Value::Primitive(PrimitiveValue::Boolean((!*a && !*b) || (*a && *b))))
    },
    _ => {
      Err(())
    }
  }
}
