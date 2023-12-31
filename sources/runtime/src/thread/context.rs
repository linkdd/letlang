use crate::{LLPid, LLInterrupt, LLValue, LLException, LLClassInstance, LLCoroutine};

use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct LLContext(Arc<Mutex<ContextData>>);

struct ContextData {
  pid: LLPid,
}

impl LLContext {
  pub async fn throw(
    &self,
    co: &LLCoroutine,
    exc: LLException,
  ) {
    let exc_info: LLValue = exc.into();
    co.yield_(LLInterrupt::Exception(exc_info)).await;
  }

  pub async fn assert_type_arity(
    &self,
    co: &LLCoroutine,
    expected: usize,
    got: usize,
  ) {
    if expected != got {
      self.throw(co, LLException::TypeArity { expected, got }).await;
    }
  }

  pub async fn assert_func_arity(
    &self,
    co: &LLCoroutine,
    expected: usize,
    got: usize,
  ) {
    if expected != got {
      self.throw(co, LLException::FuncArity { expected, got }).await;
    }
  }

  pub async fn assert_class(
    &self,
    co: &LLCoroutine,
    val: &LLValue,
    class: &LLClassInstance,
  ) {
    if !class.has(self.clone(), co, val).await {
      self.throw(co, LLException::TypeError).await;
    }
  }
}
