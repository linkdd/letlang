use crate::repr::Value;
use crate::core::{
  function::FunctionCoroutine,
  context::TaskContext,
};

use std::sync::Arc;
use tokio::sync::Mutex;


pub async fn str_concat(
  context: Arc<Mutex<TaskContext>>,
  _co: &FunctionCoroutine,
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
    (
      Value::String(a),
      _,
    ) => {
      Value::String(format!("{}{}", a, b.to_string(context.clone()).await))
    },
    (
      _,
      Value::String(b),
    ) => {
      Value::String(format!("{}{}", a.to_string(context.clone()).await, b))
    },
    _ => {
      Value::String(format!(
        "{}{}",
        a.to_string(context.clone()).await,
        b.to_string(context.clone()).await,
      ))
    }
  }
}
