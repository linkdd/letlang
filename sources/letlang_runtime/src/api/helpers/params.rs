use crate::api::*;


pub async fn assert_param_count(
  co: &FunctionCoroutine,
  context: Arc<Mutex<TaskContext>>,
  expected: usize,
  got: usize,
) {
  if expected != got {
    let type_error_atom = async {
      let context_ref = context.lock().await;
      let atom_table = context_ref.atom_table.lock().unwrap();
      atom_table.from_repr("@type_error")
    }.await;

    let exc = Value::Tuple(Box::new([
      Value::Atom(type_error_atom),
      Value::String(format!(
        "expected {} arguments, got {}",
        expected,
        got,
      )),
    ]));
    co.yield_(FunctionInterruption::Exception(exc)).await;
    unreachable!();
  }
}
