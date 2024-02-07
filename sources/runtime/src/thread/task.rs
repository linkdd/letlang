use crate::{
  prelude::*,
  data::LLValue,
  thread::LLInterrupt,
  concurrency::LLProcessContext,
  traits::{
    LLFunctionInstance,
    LLClassInstance,
  },
};

use genawaiter::GeneratorState;

pub struct LLTask {
  pub context: LLProcessContext,
  pub type_params: Vec<LLClassInstance>,
  pub func: LLFunctionInstance,
}

impl LLTask {
  pub async fn run(self) -> Result<()> {
    let task_args: Vec<LLValue> = vec![];
    let mut block = self.func.call(
      self.context.clone(),
      self.type_params,
      task_args,
    );

    let ignored = LLValue::Bool(bool::default());
    let mut state = block.resume_with(ignored);

    loop {
      match &state {
        GeneratorState::Yielded(LLInterrupt::Effect { id, args}) => {
          state = block.resume_with(todo!());
        },
        GeneratorState::Yielded(LLInterrupt::Exception(exc)) => {
          eprintln!(
            "[FATAL] Uncaught xception in task {}: {}",
            self.context.pid().await,
            exc,
          );
          return Err(RuntimeError::UncaughtException);
        },
        GeneratorState::Complete(val) => {
          eprintln!(
            "Task {} terminated with: {}",
            self.context.pid().await,
            val,
          );
          break;
        },
      }
    }

    Ok(())
  }
}
