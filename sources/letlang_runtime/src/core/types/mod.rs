use crate::core::{context::TaskContext, function::FunctionCoroutine};
use crate::repr::Value;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
pub trait Type : Sync + Send + std::fmt::Debug {
  async fn to_string(&self, context: Arc<Mutex<TaskContext>>) -> String;
  async fn has(&self, context: Arc<Mutex<TaskContext>>, co: &FunctionCoroutine, llval: &Value) -> bool;
}
