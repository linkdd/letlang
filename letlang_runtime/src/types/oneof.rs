use crate::{Value, Type};

pub struct OneOfType<'t> {
  pub lltypes: Vec<&'t dyn Type>,
}

impl<'t> Type for OneOfType<'t> {
  fn has(&self, llval: &Value) -> bool {
    for lltype in self.lltypes.iter() {
      if lltype.has(llval) {
        return true;
      }
    }

    false
  }
}
