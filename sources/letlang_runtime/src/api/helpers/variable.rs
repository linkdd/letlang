use crate::api::*;


pub async fn assert_defined<'scope>(
  co: &FunctionCoroutine,
  context: Arc<Mutex<TaskContext>>,
  locals: &mut Locals<'scope>,
  symbol: &'static str,
) -> Value {
  match locals.lookup_symbol(symbol) {
    None => {
      let err = OperationError::Undefined(symbol.to_string());
      let exc = err.to_letlang_value(context.clone()).await;

      co.yield_(FunctionInterruption::Exception(exc)).await;
      unreachable!();
    },
    Some(val) => {
      val.clone()
    }
  }
}
