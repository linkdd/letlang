use crate::core::{TaskContext, function::FunctionCoroutine, type_trait::Type};
use crate::repr::Value;

use async_trait::async_trait;

#[derive(Debug)]
pub struct TupleType {
  pub members_types: Vec<Box<dyn Type>>,
}

#[async_trait]
impl Type for TupleType {
  fn to_string(&self, context: &mut TaskContext) -> String {
    let members = self.members_types
      .iter()
      .map(|lltype| lltype.to_string(context))
      .collect::<Vec<String>>()
      .join(", ");

    format!("({})", members)
  }

  async fn has(&self, context: &mut TaskContext, co: &FunctionCoroutine, llval: &Value) -> bool {
    match llval {
      Value::Tuple(members) => {
        if members.len() != self.members_types.len() {
          return false;
        }

        let pairs =
          members.iter()
          .zip(self.members_types.iter());

        for (member, member_type) in pairs {
          if !member_type.has(context, co, member).await {
            return false;
          }
        }

        true
      },
      _ => false,
    }
  }
}
