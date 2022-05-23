pub use genawaiter::sync::{Gen, Co};

use crate::{Value, Context};

#[derive(Clone)]
pub enum FunctionInterruption {
  Effect { name: String, args: Vec<Value> },
  Exception(Value),
}

use std::sync::{Arc, Mutex};
use std::pin::Pin;
use std::future::Future;

pub type FunctionContinuation = Gen<
  FunctionInterruption,
  Value,
  Pin<Box<(dyn Future<Output = Value> + Send + 'static)>>,
>;

pub type FunctionCoroutine = Co<FunctionInterruption, Value>;

pub trait Function {
  fn call(&self, context: Arc<Mutex<Context>>, args: Vec<Value>) -> FunctionContinuation;
}
