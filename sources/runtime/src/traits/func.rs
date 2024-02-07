use crate::{
  data::LLValue,
  concurrency::LLProcessContext,
  thread::LLContinuation,
  traits::LLClassInstance,
};

pub type LLFunctionInstance = Box<dyn LLFunction>;

pub trait LLFunction: Sync + Send {
  fn call(
    &self,
    ctx: LLProcessContext,
    type_params: Vec<LLClassInstance>,
    call_params: Vec<LLValue>,
  ) -> LLContinuation;
}
