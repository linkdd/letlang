use genawaiter::GeneratorState;

use crate::prelude::*;
use crate::builtins;
use crate::repr::Value;
use crate::core::{
  context::TaskContext,
  function::{Function, FunctionInterruption},
};

use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn run(
  context: TaskContext,
  func: Box<dyn Function>,
) -> Result<()> {
  let context_cell = Arc::new(Mutex::new(context));

  let task_args: Vec<Value> = vec![];
  let mut block = func.call(context_cell.clone(), task_args);

  let ignored = Value::Boolean(bool::default());
  let mut state = block.resume_with(ignored);

  loop {
    match &state {
      GeneratorState::Yielded(FunctionInterruption::Effect { name, args }) => {
        let val = builtins::effects::dispatch(
          context_cell.clone(),
          name,
          args,
        ).await?;
        state = block.resume_with(val);
      },
      GeneratorState::Yielded(FunctionInterruption::Exception(exc)) => {
        eprintln!(
          "[FATAL] Uncaught exception in task {:?}: {}",
          async {
            let context = context_cell.lock().await;
            context.pid.clone()
          }.await,
          exc.to_string(context_cell.clone()).await,
        );
        return Err(RuntimeError::UncaughtException);
      },
      GeneratorState::Complete(val) => {
        eprintln!(
          "Task {:?} terminated with: {}",
          async {
            let context = context_cell.lock().await;
            context.pid.clone()
          }.await,
          val.to_string(context_cell.clone()).await,
        );
        break;
      }
    }
  }

  Ok(())
}
