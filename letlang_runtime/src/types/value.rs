use crate::{FunctionCoroutine, Context, Value, PrimitiveValue, Type};
use std::sync::{Arc, Mutex};
use async_trait::async_trait;

pub struct ValueType {
  pub llval: PrimitiveValue,
}

#[async_trait]
impl Type for ValueType {
  fn to_string(&self, context: Arc<Mutex<Context>>) -> String {
    context.lock().unwrap().format_primitive_value(&self.llval)
  }

  async fn has(&self, _co: &FunctionCoroutine, _context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    match llval {
      Value::Primitive(pval) => {
        *pval == self.llval
      },
      _ => false,
    }
  }
}
