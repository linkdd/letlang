use crate::{Value, Type};

pub struct AllOfType<'t> {
  pub lltypes: Vec<&'t dyn Type>,
}

impl<'t> Type for AllOfType<'t> {
  fn has(&self, llval: &Value) -> bool {
    for lltype in self.lltypes.iter() {
      if !lltype.has(llval) {
        return false;
      }
    }

    true
  }
}
