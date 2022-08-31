use genawaiter::GeneratorState;

use crate::prelude::*;
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
        let arg_list = Value::Tuple(args.clone().into_boxed_slice());

        match name.as_str() {
          "debug" => {
            let context = context_cell.lock().await;
            println!("{}", arg_list.to_string(context_cell.clone()).await);

            let ok = Value::Atom({
              let atom_table = context.atom_table.lock().unwrap();
              atom_table.from_repr("@ok")
            });
            state = block.resume_with(ok);
          }
          _ => {
            eprintln!(
              "[FATAL] Unknown effect in task {:?}: {}{}",
              async {
                let context = context_cell.lock().await;
                context.pid.clone()
              }.await,
              name,
              arg_list.to_string(context_cell.clone()).await,
            );
            return Err(RuntimeError::EffectNotImplemented);
          }
        }
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
