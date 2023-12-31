use crate::{LLValue, LLInterrupt};

use genawaiter::{
  GeneratorState,
  sync::Gen,
};
use std::{
  pin::Pin,
  future::Future,
};

pub type LLContinuation = Gen<
  LLInterrupt,
  LLValue,
  Pin<Box<(dyn Future<Output = LLValue> + Send + 'static)>>,
>;

pub type LLContinuationState = GeneratorState<LLInterrupt, LLValue>;
