mod io_inspect;
mod io_println;
mod io_readline;

use crate::prelude::*;
use crate::repr::Value;
use crate::core::context::TaskContext;

use std::sync::Arc;
use tokio::sync::Mutex;


pub async fn dispatch(
  context: Arc<Mutex<TaskContext>>,
  name: &str,
  args: &Vec<Value>,
) -> Result<Value> {
  match name {
    "std::io::__inspect" => {
      io_inspect::run(context.clone(), args).await
    },
    "std::io::__println" => {
      io_println::run(context.clone(), args).await
    },
    "std::io::__readline" => {
      io_readline::run(context.clone(), args).await
    },
    _ => {
      let arg_list = Value::Tuple(args.clone().into_boxed_slice());

      eprintln!(
        "[FATAL] Unknown effect in task {:?}: {}{}",
        async {
          let ctx = context.lock().await;
          ctx.pid.clone()
        }.await,
        name,
        arg_list.to_string(context.clone()).await,
      );
      return Err(RuntimeError::EffectNotImplemented);
    }
  }
}