use crate::builtins::prelude::*;
use crate::repr::Value;
use crate::core::{
  function::{FunctionCoroutine, FunctionInterruption},
  context::TaskContext,
};

use std::sync::Arc;
use tokio::sync::Mutex;


pub async fn throw(
  _context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  exc: &Value,
) -> Value {
  co.yield_(FunctionInterruption::Exception(exc.clone())).await;
  unreachable!();
}

pub async fn neg(
  context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  val: &Value,
) -> Value {
  match val {
    Value::Number(n) => Value::Number(-n),
    _ => {
      let err = OperationError::TypeError {
        expected: "number".to_string(),
        got: val.to_string(context.clone()).await,
      };
      let exc = err.to_letlang_value(context.clone()).await;
      co.yield_(FunctionInterruption::Exception(exc)).await;
      unreachable!();
    }
  }
}

pub async fn logical_not(
  context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  val: &Value,
) -> Value {
  match val {
    Value::Boolean(b) => Value::Boolean(!b),
    _ => {
      let err = OperationError::TypeError {
        expected: "bool".to_string(),
        got: val.to_string(context.clone()).await,
      };
      let exc = err.to_letlang_value(context.clone()).await;
      co.yield_(FunctionInterruption::Exception(exc)).await;
      unreachable!();
    }
  }
}

pub async fn bitwise_not(
  context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  val: &Value,
) -> Value {
  match val {
    Value::Number(n) => Value::Number(!(*n as i32) as f64),
    _ => {
      let err = OperationError::TypeError {
        expected: "number".to_string(),
        got: val.to_string(context.clone()).await,
      };
      let exc = err.to_letlang_value(context.clone()).await;
      co.yield_(FunctionInterruption::Exception(exc)).await;
      unreachable!();
    }
  }
}
