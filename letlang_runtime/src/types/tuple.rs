use crate::{Value, Type};

pub struct TupleType<'t> {
  pub members_types: Vec<&'t dyn Type>,
}

impl<'t> Type for TupleType<'t> {
  fn has(&self, llval: &Value) -> bool {
    match llval {
      Value::Tuple(members) => {
        if members.len() != self.members_types.len() {
          return false;
        }

        let pairs =
          members.iter()
          .zip(self.members_types.iter());

        for (member, member_type) in pairs {
          if !member_type.has(member) {
            return false;
          }
        }

        true
      },
      _ => false,
    }
  }
}
