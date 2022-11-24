use crate::core::{
  context::TaskContext,
  utils::Locals,
};
use crate::repr::Value;

use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;


#[async_trait]
pub trait Pattern: Send + Sync {
  async fn match_(
    &self,
    context: Arc<Mutex<TaskContext>>,
    locals: &mut Locals,
    value: &Value,
  ) -> Result<(), ()>;
}
