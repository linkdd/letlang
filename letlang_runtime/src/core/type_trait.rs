use crate::core::{context::TaskContext, function::FunctionCoroutine};
use crate::repr::Value;

use async_trait::async_trait;

#[async_trait]
pub trait Type : Sync + Send + std::fmt::Debug {
  fn to_string(&self, context: &mut TaskContext) -> String;
  async fn has(&self, context: &mut TaskContext, co: &FunctionCoroutine, llval: &Value) -> bool;
}
