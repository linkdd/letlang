use crate::{Value, Type};

pub struct NotType<'t> {
  pub lltype: &'t dyn Type,
}

impl<'t> Type for NotType<'t> {
  fn has(&self, llval: &Value) -> bool {
    !self.lltype.has(llval)
  }
}
