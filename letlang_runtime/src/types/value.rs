use crate::{Value, PrimitiveValue, Type};

pub struct ValueType {
  pub llval: PrimitiveValue,
}

impl Type for ValueType {
  fn has(&self, llval: &Value) -> bool {
    match llval {
      Value::Primitive(pval) => {
        *pval == self.llval
      },
      _ => false,
    }
  }
}
