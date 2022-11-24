use crate::prelude::*;
use crate::repr::Value;
use crate::core::context::TaskContext;

use std::sync::Arc;
use tokio::sync::Mutex;


pub async fn run(
  context: Arc<Mutex<TaskContext>>,
  args: &Vec<Value>,
) -> Result<Value> {
  let arg_list = Value::Tuple(args.clone().into_boxed_slice());

  println!("{}", arg_list.to_string(context.clone()).await);

  let ok_atom = Value::Atom({
    let ctx = context.lock().await;
    let atom_table = ctx.atom_table.lock().unwrap();
    atom_table.from_repr("@ok")
  });

  Ok(ok_atom)
}
