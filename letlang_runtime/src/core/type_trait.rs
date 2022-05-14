use crate::{Context, Value};
use std::sync::{Arc, Mutex};

pub trait Type : Sync {
  fn to_string(&self, context: Arc<Mutex<Context>>) -> String;
  fn has(&self, context: Arc<Mutex<Context>>, llval: &Value) -> bool;
}
