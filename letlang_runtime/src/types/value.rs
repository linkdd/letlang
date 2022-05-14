use crate::{Context, Value, PrimitiveValue, Type};
use std::sync::{Arc, Mutex};

pub struct ValueType {
  pub llval: PrimitiveValue,
}

impl Type for ValueType {
  fn to_string(&self, context: Arc<Mutex<Context>>) -> String {
    context.lock().unwrap().format_primitive_value(&self.llval)
  }

  fn has(&self, _context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    match llval {
      Value::Primitive(pval) => {
        *pval == self.llval
      },
      _ => false,
    }
  }
}
