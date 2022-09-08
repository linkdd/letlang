use crate::core::pattern::Pattern;
use crate::core::{
  context::TaskContext,
  utils::Locals,
};
use crate::repr::Value;

use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;


pub struct TuplePattern {
  pub members: Vec<Box<dyn Pattern>>,
}

#[async_trait]
impl Pattern for TuplePattern {
  async fn match_(
    &self,
    context: Arc<Mutex<TaskContext>>,
    locals: &mut Locals,
    value: &Value,
  ) -> Result<(), ()> {
    if let Value::Tuple(members) = value {
      if self.members.len() != members.len() {
        return Err(());
      }

      for (idx, member_pattern) in self.members.iter().enumerate() {
        let member_val = &members[idx];

        member_pattern.match_(context.clone(), locals, member_val).await?;
      }

      Ok(())
    }
    else {
      Err(())
    }
  }
}
