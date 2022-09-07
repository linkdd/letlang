use crate::core::{
  function::FunctionCoroutine,
  context::TaskContext,
  utils::Locals,
};
use crate::repr::Value;

use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;


#[async_trait]
pub trait Pattern {
  async fn match_(
    &self,
    co: &FunctionCoroutine,
    context: Arc<Mutex<TaskContext>>,
    locals: &mut Locals,
    value: &Value,
  );
}
