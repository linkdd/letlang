use crate::{Context, Value, Type};
use std::sync::{Arc, Mutex};

pub struct AllOfType<'t> {
  pub lltypes: Vec<&'t dyn Type>,
}

impl<'t> Type for AllOfType<'t> {
  fn to_string(&self, context: Arc<Mutex<Context>>) -> String {
    self.lltypes
      .iter()
      .map(|lltype| lltype.to_string(context.clone()))
      .collect::<Vec<String>>()
      .join(" & ")
  }

  fn has(&self, context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    for lltype in self.lltypes.iter() {
      if !lltype.has(context.clone(), llval) {
        return false;
      }
    }

    true
  }
}
