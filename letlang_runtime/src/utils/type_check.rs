use crate::*;

pub async fn assert_type(
  co: &FunctionCoroutine,
  context: Arc<Mutex<Context>>,
  lltype: &dyn Type,
  llval: &Value,
  err_reason: String,
) {
  if !lltype.has(co, context.clone(), llval).await {
    let exc = Value::Tuple(vec![
      Value::Primitive(PrimitiveValue::Atom(
        context.lock().unwrap().get_atom("@type_error")
      )),
      Value::Primitive(PrimitiveValue::String(err_reason)),
    ]);
    co.yield_(FunctionInterruption::Exception(exc)).await;
    unreachable!();
  }
}
