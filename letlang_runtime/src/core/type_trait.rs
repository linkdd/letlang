use crate::{Context, Value, FunctionCoroutine};
use std::sync::{Arc, Mutex};
use async_trait::async_trait;

#[async_trait]
pub trait Type : Sync + Send {
  fn to_string(&self, context: Arc<Mutex<Context>>) -> String;
  async fn has(&self, co: &FunctionCoroutine, context: Arc<Mutex<Context>>, llval: &Value) -> bool;
}
