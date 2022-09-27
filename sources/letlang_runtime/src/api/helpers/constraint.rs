use crate::api::*;


pub async fn assert_constraints<'scope>(
  co: &FunctionCoroutine,
  context: Arc<Mutex<TaskContext>>,
  locals: &Locals<'scope>,
) {
  for constraint in locals.iter_constraints() {
    let res = constraint.check(co, context.clone(), locals).await;
    if let Err(symbol) = res {
      let err = OperationError::ConstraintError(symbol);
      let exc = err.to_letlang_value(context.clone()).await;

      co.yield_(FunctionInterruption::Exception(exc)).await;
      unreachable!();
    }
  }
}
