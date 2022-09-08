use crate::prelude::*;
use crate::repr::Value;
use crate::core::context::TaskContext;

use std::sync::Arc;
use tokio::sync::Mutex;
use std::io::Write;


pub async fn run(
  context: Arc<Mutex<TaskContext>>,
  args: &Vec<Value>,
) -> Result<Value> {
  let prompt_val = args
    .first()
    .ok_or(RuntimeError::Other("effect __io_readline expected 1 argument"))?;

  if let Value::String(prompt) = prompt_val {
    print!("{} ", prompt);
    std::io::stdout().flush().unwrap();
  }
  else {
    unreachable!("Type checking should have proven this value is a string");
  }

  let line = { std::io::stdin().lines().next() };

  let retval = match line {
    None => {
      let error_atom = Value::Atom({
        let ctx = context.lock().await;
        let atom_table = ctx.atom_table.lock().unwrap();
        atom_table.from_repr("@error")
      });

      let eof_atom = Value::Atom({
        let ctx = context.lock().await;
        let atom_table = ctx.atom_table.lock().unwrap();
        atom_table.from_repr("@eof")
      });

      Value::Tuple(Box::new([error_atom, eof_atom]))
    },
    Some(Ok(buf)) => {
      let ok_atom = Value::Atom({
        let ctx = context.lock().await;
        let atom_table = ctx.atom_table.lock().unwrap();
        atom_table.from_repr("@ok")
      });

      let buf_val = Value::String(buf);

      Value::Tuple(Box::new([ok_atom, buf_val]))
    },
    Some(Err(err)) => {
      let error_atom = Value::Atom({
        let ctx = context.lock().await;
        let atom_table = ctx.atom_table.lock().unwrap();
        atom_table.from_repr("@error")
      });

      let io_atom = Value::Atom({
        let ctx = context.lock().await;
        let atom_table = ctx.atom_table.lock().unwrap();
        atom_table.from_repr("@io")
      });

      let err_val = Value::String(err.to_string());

      let reason = Value::Tuple(Box::new([io_atom, err_val]));

      Value::Tuple(Box::new([error_atom, reason]))
    }
  };

  Ok(retval)
}
