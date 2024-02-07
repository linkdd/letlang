use crate::{
  prelude::*,
  data::{LLValue, LLPid},
};

use tokio::sync::mpsc;

#[derive(Clone)]
pub enum LLSignal {
  Message(LLPid, LLValue),
  Exited(LLPid, LLValue),
}

#[derive(Clone)]
pub struct LLSignalSender(mpsc::Sender<LLSignal>);

pub struct LLSignalReceiver(mpsc::Receiver<LLSignal>);

pub struct LLChannel;

impl LLChannel {
  pub fn new() -> (LLSignalSender, LLSignalReceiver) {
    let (tx, rx) = mpsc::channel(32);
    (LLSignalSender(tx), LLSignalReceiver(rx))
  }
}

impl LLSignalSender {
  pub async fn send(&self, signal: LLSignal) -> Result<()> {
    let Self(tx) = self;
    tx.send(signal).await?;
    Ok(())
  }
}

impl LLSignalReceiver {
  pub async fn receive(&mut self) -> Result<LLSignal> {
    let Self(rx) = self;
    rx.recv().await.ok_or(RuntimeError::ChannelReceiveError)
  }
}
