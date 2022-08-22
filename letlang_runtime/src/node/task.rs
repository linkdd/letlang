use genawaiter::GeneratorState;

use crate::prelude::*;
use crate::repr::Value;
use crate::core::{
  TaskContext,
  function::{Function, FunctionInterruption},
};

pub async fn run(
  mut context: TaskContext,
  func: Box<dyn Function>,
) -> Result<()> {
  let task_args: Vec<Value> = vec![];
  let mut block = func.call(&mut context, task_args);

  let ignored = Value::Boolean(bool::default());
  let mut state = block.resume_with(ignored);

  loop {
    match &state {
      GeneratorState::Yielded(FunctionInterruption::Effect { name, args }) => {
        let arg_list = Value::Tuple(args.clone().into_boxed_slice());

        match name.as_str() {
          "debug" => {
            println!("{}", arg_list);

            let ok = Value::Atom(
              context.builtin_atoms.lock().unwrap().get("@ok")
            );
            state = block.resume_with(ok);
          }
          _ => {
            eprintln!("[FATAL] Unknown effect: {}{}", name, arg_list);
            return Err(RuntimeError::EffectNotImplemented);
          }
        }
      },
      GeneratorState::Yielded(FunctionInterruption::Exception(exc)) => {
        eprintln!("[FATAL] Uncaught exception: {}", exc);
        return Err(RuntimeError::UncaughtException);
      },
      GeneratorState::Complete(val) => {
        eprintln!("Task terminated with: {:?}", val);
        break;
      }
    }
  }

  Ok(())
}
