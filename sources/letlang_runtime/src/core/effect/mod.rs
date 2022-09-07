use crate::core::{context::TaskContext, function::FunctionCoroutine};
use crate::repr::Value;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
pub trait Effect : Sync + Send + std::fmt::Debug {
  async fn call(
    &self,
    context: Arc<Mutex<TaskContext>>,
    co: &FunctionCoroutine,
    args: Vec<Value>,
  ) -> Value;
}
