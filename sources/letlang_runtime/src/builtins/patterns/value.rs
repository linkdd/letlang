use crate::core::pattern::Pattern;
use crate::core::{
  context::TaskContext,
  utils::Locals,
};
use crate::repr::Value;

use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;


pub struct ValuePattern {
  pub llval: Value,
}

#[async_trait]
impl Pattern for ValuePattern {
  async fn match_(
    &self,
    _context: Arc<Mutex<TaskContext>>,
    _locals: &mut Locals,
    value: &Value,
  ) -> Result<(), ()> {
    if value != &self.llval {
      Err(())
    }
    else {
      Ok(())
    }
  }
}
