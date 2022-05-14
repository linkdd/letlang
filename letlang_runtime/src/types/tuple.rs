use crate::{Context, Value, Type};
use std::sync::{Arc, Mutex};

pub struct TupleType<'t> {
  pub members_types: Vec<&'t dyn Type>,
}

impl<'t> Type for TupleType<'t> {
  fn to_string(&self, context: Arc<Mutex<Context>>) -> String {
    let members = self.members_types
      .iter()
      .map(|lltype| lltype.to_string(context.clone()))
      .collect::<Vec<String>>()
      .join(", ");

    format!("({})", members)
  }

  fn has(&self, context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    match llval {
      Value::Tuple(members) => {
        if members.len() != self.members_types.len() {
          return false;
        }

        let pairs =
          members.iter()
          .zip(self.members_types.iter());

        for (member, member_type) in pairs {
          if !member_type.has(context.clone(), member) {
            return false;
          }
        }

        true
      },
      _ => false,
    }
  }
}
