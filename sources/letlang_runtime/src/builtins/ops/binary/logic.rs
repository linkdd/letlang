use crate::builtins::prelude::*;
use crate::repr::Value;
use crate::core::{
  function::{FunctionCoroutine, FunctionInterruption},
  context::TaskContext,
};

use std::sync::Arc;
use tokio::sync::Mutex;


pub async fn and(
  context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  a: &Value,
  b: &Value,
) -> Value {
  match (a, b) {
    (
      Value::Boolean(a),
      Value::Boolean(b),
    ) => {
      Value::Boolean(*a && *b)
    },
    _ => {
      let err = OperationError::TypeError {
        expected: "(boolean, boolean)".to_string(),
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


pub async fn or(
  context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  a: &Value,
  b: &Value,
) -> Value {
  match (a, b) {
    (
      Value::Boolean(a),
      Value::Boolean(b),
    ) => {
      Value::Boolean(*a || *b)
    },
    _ => {
      let err = OperationError::TypeError {
        expected: "(boolean, boolean)".to_string(),
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
