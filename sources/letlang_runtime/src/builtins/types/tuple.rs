use crate::core::{context::TaskContext, function::FunctionCoroutine, types::Type};
use crate::repr::Value;

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;


#[derive(Debug)]
pub struct TupleType {
  pub members_types: Vec<Box<dyn Type>>,
}

#[async_trait]
impl Type for TupleType {
  async fn to_string(&self, context: Arc<Mutex<TaskContext>>) -> String {
    let mut members = vec![];

    for lltype in self.members_types.iter() {
      members.push(lltype.to_string(context.clone()).await);
    }

    format!("({})", members.join(", "))
  }

  async fn has(&self, context: Arc<Mutex<TaskContext>>, co: &FunctionCoroutine, llval: &Value) -> bool {
    match llval {
      Value::Tuple(members) => {
        if members.len() != self.members_types.len() {
          return false;
        }

        let pairs =
          members.iter()
          .zip(self.members_types.iter());

        for (member, member_type) in pairs {
          if !member_type.has(context.clone(), co, member).await {
            return false;
          }
        }

        true
      },
      _ => false,
    }
  }
}
