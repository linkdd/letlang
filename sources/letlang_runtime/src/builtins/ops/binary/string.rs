use crate::builtins::result::*;
use crate::repr::Value;
use crate::core::{
  function::{FunctionCoroutine, FunctionInterruption},
  context::TaskContext,
};

use std::sync::Arc;
use tokio::sync::Mutex;


pub async fn str_concat(
  context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  a: &Value,
  b: &Value,
) -> Value {
  match (a, b) {
    (
      Value::String(a),
      Value::String(b),
    ) => {
      Value::String(format!("{}{}", a, b))
    },
    _ => {
      let err = OperationError::TypeError {
        expected: "(string, string)".to_string(),
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