use crate::{
  data::LLValue,
  concurrency::LLProcessContext,
  thread::LLCoroutine,
};

use async_trait::async_trait;

pub type LLClassInstance = Box<dyn LLClass>;

#[async_trait]
pub trait LLClass: Sync + Send {
  async fn has(
    &self,
    ctx: LLProcessContext,
    co: &LLCoroutine,
    val: &LLValue,
  ) -> bool;
}
