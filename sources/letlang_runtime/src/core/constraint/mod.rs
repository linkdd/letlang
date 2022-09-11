use crate::core::{context::TaskContext, function::FunctionCoroutine};
use crate::core::utils::Locals;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;


#[async_trait]
pub trait Constraint : Sync + Send + std::fmt::Debug {
  async fn check(
    &self,
    co: &FunctionCoroutine,
    context: Arc<Mutex<TaskContext>>,
    locals: &Locals,
  ) -> Result<(), String>;
}
