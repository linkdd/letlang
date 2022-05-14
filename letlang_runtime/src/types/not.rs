use crate::{Context, Value, Type};
use std::sync::{Arc, Mutex};

pub struct NotType<'t> {
  pub lltype: &'t dyn Type,
}

impl<'t> Type for NotType<'t> {
  fn to_string(&self, context: Arc<Mutex<Context>>) -> String {
    format!("!{}", self.lltype.to_string(context))
  }

  fn has(&self, context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    !self.lltype.has(context, llval)
  }
}
