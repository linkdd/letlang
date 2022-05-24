use crate::{FunctionCoroutine, Context, Value, Type};
use std::sync::{Arc, Mutex};
use async_trait::async_trait;

pub struct NotType {
  pub lltype: Box<dyn Type>,
}

#[async_trait]
impl Type for NotType {
  fn to_string(&self, context: Arc<Mutex<Context>>) -> String {
    format!("!{}", self.lltype.to_string(context))
  }

  async fn has(&self, co: &FunctionCoroutine, context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    !self.lltype.has(co, context, llval).await
  }
}
