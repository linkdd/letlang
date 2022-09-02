use crate::prelude::*;
use crate::repr::{Pid, Value};

use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum Signal {
  Message(Pid, Value),
  Exited(Pid, Value),
}

#[derive(Debug, Clone)]
pub struct SignalSender(mpsc::Sender<Signal>);

#[derive(Debug)]
pub struct SignalReceiver(mpsc::Receiver<Signal>);


pub fn channel() -> (SignalSender, SignalReceiver) {
  let (tx, rx) = mpsc::channel(32);
  (SignalSender(tx), SignalReceiver(rx))
}


impl SignalSender {
  pub async fn send(&self, req: Signal) -> Result<()> {
    let SignalSender(tx) = self;
    tx.send(req).await.map_err(|_err| {
      RuntimeError::ChannelSendFailed
    })?;
    Ok(())
  }
}

impl SignalReceiver {
  pub async fn receive(&mut self) -> Option<Signal> {
    let SignalReceiver(rx) = self;
    rx.recv().await
  }
}
