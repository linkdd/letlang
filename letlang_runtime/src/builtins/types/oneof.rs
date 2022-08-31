use crate::core::{context::TaskContext, function::FunctionCoroutine, type_trait::Type};
use crate::repr::Value;

use async_trait::async_trait;

#[derive(Debug)]
pub struct OneOfType {
  pub lltypes: Vec<Box<dyn Type>>,
}

#[async_trait]
impl Type for OneOfType {
  fn to_string(&self, context: &mut TaskContext) -> String {
    self.lltypes
      .iter()
      .map(|lltype| lltype.to_string(context))
      .collect::<Vec<String>>()
      .join(" | ")
  }

  async fn has(&self, context: &mut TaskContext, co: &FunctionCoroutine, llval: &Value) -> bool {
    for lltype in self.lltypes.iter() {
      if lltype.has(context, co, llval).await {
        return true;
      }
    }

    false
  }
}
