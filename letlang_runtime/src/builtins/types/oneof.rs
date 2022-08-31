use crate::core::{context::TaskContext, function::FunctionCoroutine, type_trait::Type};
use crate::repr::Value;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;


#[derive(Debug)]
pub struct OneOfType {
  pub lltypes: Vec<Box<dyn Type>>,
}

#[async_trait]
impl Type for OneOfType {
  async fn to_string(&self, context: Arc<Mutex<TaskContext>>) -> String {
    let mut types_repr = vec![];

    for lltype in self.lltypes.iter() {
      types_repr.push(lltype.to_string(context.clone()).await);
    }

    types_repr.join(" | ")
  }

  async fn has(&self, context: Arc<Mutex<TaskContext>>, co: &FunctionCoroutine, llval: &Value) -> bool {
    for lltype in self.lltypes.iter() {
      if lltype.has(context.clone(), co, llval).await {
        return true;
      }
    }

    false
  }
}
