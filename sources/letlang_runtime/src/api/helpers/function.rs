use crate::api::*;
use genawaiter::GeneratorState;


pub async fn call_function(
  co: &FunctionCoroutine,
  context: Arc<Mutex<TaskContext>>,
  func: &dyn Function,
  args: Vec<Value>,
) -> Value {
  let mut block = func.call(context.clone(), args);

  let ignored = Value::Boolean(bool::default());
  let mut state = block.resume_with(ignored);

  let func_result = loop {
    match state {
      GeneratorState::Yielded(interrupt) => {
        let res = co.yield_(interrupt.clone()).await;
        state = block.resume_with(res);
      },
      GeneratorState::Complete(res) => {
        break res;
      }
    }
  };

  func_result
}
