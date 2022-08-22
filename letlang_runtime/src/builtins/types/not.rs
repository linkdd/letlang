use crate::core::{TaskContext, function::FunctionCoroutine, type_trait::Type};
use crate::repr::Value;

use async_trait::async_trait;

#[derive(Debug)]
pub struct NotType {
  pub lltype: Box<dyn Type>,
}

#[async_trait]
impl Type for NotType {
  fn to_string(&self, context: &mut TaskContext) -> String {
    format!("!{}", self.lltype.to_string(context))
  }

  async fn has(&self, context: &mut TaskContext, co: &FunctionCoroutine, llval: &Value) -> bool {
    !self.lltype.has(context, co, llval).await
  }
}
