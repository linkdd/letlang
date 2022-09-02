use crate::api::*;

pub async fn assert_type(
  co: &FunctionCoroutine,
  context: Arc<Mutex<TaskContext>>,
  lltype: &dyn Type,
  llval: &Value,
) {
  if !lltype.has(context.clone(), co, llval).await {
    let type_error_atom = async {
      let context_ref = context.lock().await;
      let atom_table = context_ref.atom_table.lock().unwrap();
      atom_table.from_repr("@type_error")
    }.await;

    let exc = Value::Tuple(Box::new([
      Value::Atom(type_error_atom),
      Value::String(format!(
        "expected type {}, got value {}",
        lltype.to_string(context.clone()).await,
        llval.to_string(context.clone()).await,
      )),
    ]));

    co.yield_(FunctionInterruption::Exception(exc)).await;
    unreachable!();
  }
}
