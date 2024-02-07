use crate::{
  data::LLPid,
  concurrency::LLSignalSender,
};

pub struct LLProcessHandle {
  pub links: Vec<LLPid>,
  pub mbox_tx: LLSignalSender,
}

impl LLProcessHandle {
  pub fn new(mbox_tx: LLSignalSender) -> Self {
    Self { links: vec![], mbox_tx }
  }
}
