use crate::prelude::*;

use tokio::sync::oneshot;

#[derive(Debug)]
pub struct ResponseSender(oneshot::Sender<Result<()>>);

#[derive(Debug)]
pub struct ResponseReceiver(oneshot::Receiver<Result<()>>);


pub fn channel() -> (ResponseSender, ResponseReceiver) {
  let (tx, rx) = oneshot::channel();
  (ResponseSender(tx), ResponseReceiver(rx))
}


impl ResponseSender {
  pub fn send(self, resp: Result<()>) -> Result<()> {
    let ResponseSender(tx) = self;
    tx.send(resp).map_err(|_err| {
      RuntimeError::ChannelSendFailed
    })?;
    Ok(())
  }
}

impl ResponseReceiver {
  pub async fn receive(&mut self) -> Result<()> {
    let ResponseReceiver(rx) = self;
    rx.await.map_err(|_err| RuntimeError::ChannelRecvFailed)?
  }
}
