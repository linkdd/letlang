use crate::api::*;
use crate::builtins::result::*;

pub async fn assert_type(
  co: &FunctionCoroutine,
  context: Arc<Mutex<TaskContext>>,
  lltype: &dyn Type,
  llval: &Value,
) {
  if !lltype.has(context.clone(), co, llval).await {
    let err = OperationError::TypeError {
      expected: lltype.to_string(context.clone()).await,
      got: llval.to_string(context.clone()).await,
    };

    let exc = err.to_letlang_value(context.clone()).await;

    co.yield_(FunctionInterruption::Exception(exc)).await;
    unreachable!();
  }
}
