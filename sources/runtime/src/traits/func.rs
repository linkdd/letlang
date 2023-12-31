use crate::{LLContext, LLContinuation, LLClassInstance, LLValue};

pub trait LLFunction: Sync + Send {
  fn call(
    &self,
    ctx: LLContext,
    type_params: Vec<LLClassInstance>,
    call_params: Vec<LLValue>,
  ) -> LLContinuation;
}
