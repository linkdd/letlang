use crate::{
  data::LLPid,
  concurrency::signal::LLChannel,
};

mod context;
mod handle;

pub use self::{
  context::*,
  handle::*,
};

pub struct LLProcess;

impl LLProcess {
  pub fn new(pid: LLPid) -> (LLProcessContext, LLProcessHandle) {
    let (tx, rx) = LLChannel::new();
    (LLProcessContext::new(pid, rx), LLProcessHandle::new(tx))
  }
}
