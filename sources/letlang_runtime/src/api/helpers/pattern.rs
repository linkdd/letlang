use crate::api::*;
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
    let match_error_atom = async {
      let context_ref = context.lock().await;
      let atom_table = context_ref.atom_table.lock().unwrap();
      atom_table.from_repr("@match_error")
    }.await;

    let exc = Value::Tuple(Box::new([
      Value::Atom(match_error_atom),
      Value::String(format!(
        "no match on lefthand side for {}",
        llval.to_string(context.clone()).await,
      )),
    ]));

    co.yield_(FunctionInterruption::Exception(exc)).await;
    unreachable!();
  }
}
