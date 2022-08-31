mod interrupt;
mod continuation;
mod coroutine;

pub use self::{
  interrupt::FunctionInterruption,
  continuation::FunctionContinuation,
  coroutine::FunctionCoroutine,
};

use crate::core::context::TaskContext;
use crate::repr::Value;

pub trait Function: Sync + Send + std::fmt::Debug {
  fn call(&self, context: &mut TaskContext, args: Vec<Value>) -> FunctionContinuation;
}
