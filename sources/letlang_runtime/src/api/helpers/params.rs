use crate::api::*;
use crate::builtins::prelude::*;


pub async fn assert_param_count(
  co: &FunctionCoroutine,
  context: Arc<Mutex<TaskContext>>,
  expected: usize,
  got: usize,
) {
  if expected != got {
    let err = OperationError::CallParamCountError { expected, got };
    let exc = err.to_letlang_value(context.clone()).await;

    co.yield_(FunctionInterruption::Exception(exc)).await;
    unreachable!();
  }
}
