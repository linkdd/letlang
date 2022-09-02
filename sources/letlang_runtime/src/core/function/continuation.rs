use crate::core::function::FunctionInterruption;
use crate::repr::Value;

use genawaiter::sync::Gen;
use std::pin::Pin;
use std::future::Future;

pub type FunctionContinuation = Gen<
  FunctionInterruption,
  Value,
  Pin<Box<(dyn Future<Output = Value> + Send + 'static)>>,
>;
