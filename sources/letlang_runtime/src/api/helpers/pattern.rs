use crate::api::*;
use crate::builtins::prelude::*;
use crate::core::pattern::Pattern;

pub async fn assert_match<'locals>(
  co: &FunctionCoroutine,
  context: Arc<Mutex<TaskContext>>,
  locals: &mut Locals<'locals>,
  pattern: &dyn Pattern,
  llval: &Value,
) {
  let res = pattern.match_(context.clone(), locals, llval).await;

  if res.is_err() {
    let err = OperationError::MatchError {
      value: llval.to_string(context.clone()).await,
    };

    let exc = err.to_letlang_value(context.clone()).await;

    co.yield_(FunctionInterruption::Exception(exc)).await;
    unreachable!();
  }
}
