use crate::builtins::result::*;
use crate::repr::Value;
use crate::core::{
  function::{FunctionCoroutine, FunctionInterruption},
  context::TaskContext,
};

use std::sync::Arc;
use tokio::sync::Mutex;


pub async fn add(
  context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  a: &Value,
  b: &Value,
) -> Value {
  match (a, b) {
    (
      Value::Number(a),
      Value::Number(b),
    ) => {
      Value::Number(a + b)
    },
    _ => {
      let err = OperationError::TypeError {
        expected: "(number, number)".to_string(),
        got: format!(
          "({}, {})",
          a.to_string(context.clone()).await,
          b.to_string(context.clone()).await,
        )
      };
      let exc = err.to_letlang_value(context.clone()).await;
      co.yield_(FunctionInterruption::Exception(exc)).await;
      unreachable!();
    }
  }
}

pub async fn sub(
  context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  a: &Value,
  b: &Value,
) -> Value {
  match (a, b) {
    (
      Value::Number(a),
      Value::Number(b),
    ) => {
      Value::Number(a - b)
    },
    _ => {
      let err = OperationError::TypeError {
        expected: "(number, number)".to_string(),
        got: format!(
          "({}, {})",
          a.to_string(context.clone()).await,
          b.to_string(context.clone()).await,
        )
      };
      let exc = err.to_letlang_value(context.clone()).await;
      co.yield_(FunctionInterruption::Exception(exc)).await;
      unreachable!();
    }
  }
}

pub async fn mul(
  context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  a: &Value,
  b: &Value,
) -> Value {
  match (a, b) {
    (
      Value::Number(a),
      Value::Number(b),
    ) => {
      Value::Number(a * b)
    },
    _ => {
      let err = OperationError::TypeError {
        expected: "(number, number)".to_string(),
        got: format!(
          "({}, {})",
          a.to_string(context.clone()).await,
          b.to_string(context.clone()).await,
        )
      };
      let exc = err.to_letlang_value(context.clone()).await;
      co.yield_(FunctionInterruption::Exception(exc)).await;
      unreachable!();
    }
  }
}

pub async fn div(
  context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  a: &Value,
  b: &Value,
) -> Value {
  match (a, b) {
    (
      Value::Number(a),
      Value::Number(b),
    ) => {
      if *b == 0f64 {
        let err = OperationError::DivisionByZero;
        let exc = err.to_letlang_value(context.clone()).await;
        co.yield_(FunctionInterruption::Exception(exc)).await;
        unreachable!();
      }
      else {
        Value::Number(a / b)
      }
    },
    _ => {
      let err = OperationError::TypeError {
        expected: "(number, number)".to_string(),
        got: format!(
          "({}, {})",
          a.to_string(context.clone()).await,
          b.to_string(context.clone()).await,
        )
      };
      let exc = err.to_letlang_value(context.clone()).await;
      co.yield_(FunctionInterruption::Exception(exc)).await;
      unreachable!();
    }
  }
}

pub async fn modulo(
  context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  a: &Value,
  b: &Value,
) -> Value {
  match (a, b) {
    (
      Value::Number(a),
      Value::Number(b),
    ) => {
      Value::Number(a.rem_euclid(*b))
    },
    _ => {
      let err = OperationError::TypeError {
        expected: "(number, number)".to_string(),
        got: format!(
          "({}, {})",
          a.to_string(context.clone()).await,
          b.to_string(context.clone()).await,
        )
      };
      let exc = err.to_letlang_value(context.clone()).await;
      co.yield_(FunctionInterruption::Exception(exc)).await;
      unreachable!();
    }
  }
}

pub async fn pow(
  context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  a: &Value,
  b: &Value,
) -> Value {
  match (a, b) {
    (
      Value::Number(a),
      Value::Number(b),
    ) => {
      Value::Number(a.powf(*b))
    },
    _ => {
      let err = OperationError::TypeError {
        expected: "(number, number)".to_string(),
        got: format!(
          "({}, {})",
          a.to_string(context.clone()).await,
          b.to_string(context.clone()).await,
        )
      };
      let exc = err.to_letlang_value(context.clone()).await;
      co.yield_(FunctionInterruption::Exception(exc)).await;
      unreachable!();
    }
  }
}
