use crate::{LLContext, LLCoroutine, LLValue, async_trait};

pub type LLClassInstance = Box<dyn LLClass>;

#[async_trait]
pub trait LLClass: Sync + Send {
  async fn has(
    &self,
    ctx: LLContext,
    co: &LLCoroutine,
    val: &LLValue,
  ) -> bool;
}
