use crate::prelude::*;
use tokio::sync::oneshot;

pub struct ResponseChannel;

pub struct ResponseSender(oneshot::Sender<Result<()>>);

pub struct ResponseReceiver(oneshot::Receiver<Result<()>>);


impl ResponseChannel {
  pub fn new() -> (ResponseSender, ResponseReceiver) {
    let (tx, rx) = oneshot::channel();
    (ResponseSender(tx), ResponseReceiver(rx))
  }
}

impl ResponseSender {
  pub fn send(self, resp: Result<()>) -> Result<()> {
    let Self(tx) = self;
    tx.send(resp).map_err(|_| RuntimeError::ChannelSendError)?;
    Ok(())
  }
}

impl ResponseReceiver {
  pub async fn receive(&mut self) -> Result<()> {
    let Self(rx) = self;
    rx.await?
  }
}
