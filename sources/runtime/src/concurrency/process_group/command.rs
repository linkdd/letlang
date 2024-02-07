use crate::{
  prelude::*,
  data::LLPid,
  concurrency::signal::LLSignal,
  traits::{
    LLFunctionInstance,
    LLClassInstance,
  },
};
use super::{
  request::{Request, RequestSender},
  response::ResponseChannel,
};

pub enum Command {
  Spawn(Vec<LLClassInstance>, LLFunctionInstance),
  SendTo(LLPid, LLSignal),
  Link(LLPid, LLPid),
  Unlink(LLPid, LLPid),
  Exited(LLPid, Result<()>),
}

impl Command {
  pub async fn send(self, sender: &mut RequestSender) -> Result<()> {
    let (resp_tx, mut resp_rx) = ResponseChannel::new();
    let req = Request { respond_to: resp_tx, command: self };
    sender.send(req).await?;
    resp_rx.receive().await
  }
}
