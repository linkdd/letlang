use crate::{FunctionCoroutine, Context, Value, Type};
use std::sync::{Arc, Mutex};
use async_trait::async_trait;

pub struct AllOfType {
  pub lltypes: Vec<Box<dyn Type>>,
}

#[async_trait]
impl Type for AllOfType {
  fn to_string(&self, context: Arc<Mutex<Context>>) -> String {
    self.lltypes
      .iter()
      .map(|lltype| lltype.to_string(context.clone()))
      .collect::<Vec<String>>()
      .join(" & ")
  }

  async fn has(&self, co: &FunctionCoroutine, context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    for lltype in self.lltypes.iter() {
      if !lltype.has(co, context.clone(), llval).await {
        return false;
      }
    }

    true
  }
}
