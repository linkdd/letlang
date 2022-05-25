use crate::{FunctionCoroutine, Context, Value, PrimitiveValue};
use std::sync::{Arc, Mutex};

pub async fn eq(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
  Ok(Value::Primitive(PrimitiveValue::Boolean(
    a == b
  )))
}

pub async fn ne(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
  Ok(Value::Primitive(PrimitiveValue::Boolean(
    a != b
  )))
}

pub async fn lt(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
  match (a, b) {
    (
      Value::Primitive(PrimitiveValue::Number(a)),
      Value::Primitive(PrimitiveValue::Number(b)),
    ) => {
      Ok(Value::Primitive(PrimitiveValue::Boolean(a < b)))
    },
    _ => {
      Err(())
    }
  }
}


pub async fn lte(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
  match (a, b) {
    (
      Value::Primitive(PrimitiveValue::Number(a)),
      Value::Primitive(PrimitiveValue::Number(b)),
    ) => {
      Ok(Value::Primitive(PrimitiveValue::Boolean(a <= b)))
    },
    _ => {
      Err(())
    }
  }
}


pub async fn gte(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
  match (a, b) {
    (
      Value::Primitive(PrimitiveValue::Number(a)),
      Value::Primitive(PrimitiveValue::Number(b)),
    ) => {
      Ok(Value::Primitive(PrimitiveValue::Boolean(a >= b)))
    },
    _ => {
      Err(())
    }
  }
}


pub async fn gt(_co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, a: &Value, b: &Value) -> Result<Value, ()> {
  match (a, b) {
    (
      Value::Primitive(PrimitiveValue::Number(a)),
      Value::Primitive(PrimitiveValue::Number(b)),
    ) => {
      Ok(Value::Primitive(PrimitiveValue::Boolean(a > b)))
    },
    _ => {
      Err(())
    }
  }
}