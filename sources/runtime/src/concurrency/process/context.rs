use crate::{
  data::{
    LLPid,
    LLValue,
    LLException,
  },
  thread::{
    LLInterrupt,
    LLCoroutine,
  },
  concurrency::signal::LLSignalReceiver,
  traits::LLClassInstance,
};

use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct LLProcessContext(Arc<Mutex<ContextData>>);

struct ContextData {
  pid: LLPid,
  mbox_rx: LLSignalReceiver,
}

impl LLProcessContext {
  pub fn new(pid: LLPid, mbox_rx: LLSignalReceiver) -> Self {
    let data = ContextData { pid, mbox_rx };
    Self(Arc::new(Mutex::new(data)))
  }

  pub async fn pid(&self) -> LLPid {
    self.0.lock().await.pid.clone()
  }

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
