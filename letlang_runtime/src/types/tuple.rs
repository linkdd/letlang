use crate::{FunctionCoroutine, Context, Value, Type};
use std::sync::{Arc, Mutex};
use async_trait::async_trait;

pub struct TupleType {
  pub members_types: Vec<Box<dyn Type>>,
}

#[async_trait]
impl Type for TupleType {
  fn to_string(&self, context: Arc<Mutex<Context>>) -> String {
    let members = self.members_types
      .iter()
      .map(|lltype| lltype.to_string(context.clone()))
      .collect::<Vec<String>>()
      .join(", ");

    format!("({})", members)
  }

  async fn has(&self, co: &FunctionCoroutine, context: Arc<Mutex<Context>>, llval: &Value) -> bool {
    match llval {
      Value::Tuple(members) => {
        if members.len() != self.members_types.len() {
          return false;
        }

        let pairs =
          members.iter()
          .zip(self.members_types.iter());

        for (member, member_type) in pairs {
          if !member_type.has(co, context.clone(), member).await {
            return false;
          }
        }

        true
      },
      _ => false,
    }
  }
}
