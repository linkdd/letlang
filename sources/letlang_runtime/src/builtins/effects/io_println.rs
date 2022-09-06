use crate::prelude::*;
use crate::repr::Value;
use crate::core::context::TaskContext;

use std::sync::Arc;
use tokio::sync::Mutex;


pub async fn run(
  context: Arc<Mutex<TaskContext>>,
  args: &Vec<Value>,
) -> Result<Value> {
  let msg = args
    .first()
    .ok_or(RuntimeError::Other("effect __io_println expected 1 argument"))?;

  match msg {
    Value::String(s) => {
      println!("{}", s);
    },
    _ => {
      unreachable!("\
        type checking should have proven that this value is a string\
      ");
    }
  }

  let ok_atom = Value::Atom({
    let ctx = context.lock().await;
    let atom_table = ctx.atom_table.lock().unwrap();
    atom_table.from_repr("@ok")
  });

  Ok(ok_atom)
}
