use crate::core::{context::TaskContext, function::FunctionCoroutine, types::Type};
use crate::repr::Value;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;


#[derive(Debug)]
pub struct NotType {
  pub lltype: Box<dyn Type>,
}

#[async_trait]
impl Type for NotType {
  async fn to_string(&self, context: Arc<Mutex<TaskContext>>) -> String {
    format!("!{}", self.lltype.to_string(context.clone()).await)
  }

  async fn has(&self, context: Arc<Mutex<TaskContext>>, co: &FunctionCoroutine, llval: &Value) -> bool {
    !self.lltype.has(context.clone(), co, llval).await
  }
}
