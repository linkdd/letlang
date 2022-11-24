use crate::prelude::*;
use crate::node::{Command, response};

use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Request {
  pub respond_to: response::ResponseSender,
  pub command: Command,
}

#[derive(Debug, Clone)]
pub struct RequestSender(mpsc::Sender<Request>);

#[derive(Debug)]
pub struct RequestReceiver(mpsc::Receiver<Request>);


pub fn channel() -> (RequestSender, RequestReceiver) {
  let (tx, rx) = mpsc::channel(32);
  (RequestSender(tx), RequestReceiver(rx))
}

impl Request {
  pub async fn publish(command: Command, sender: &mut RequestSender) -> Result<()> {
    let (resp_tx, mut resp_rx) = response::channel();
    let req = Self {
      respond_to: resp_tx,
      command,
    };
    sender.send(req).await?;
    resp_rx.receive().await
  }
}

impl RequestSender {
  pub async fn send(&self, req: Request) -> Result<()> {
    let RequestSender(tx) = self;
    tx.send(req).await.map_err(|_err| {
      RuntimeError::ChannelSendFailed
    })?;
    Ok(())
  }
}

impl RequestReceiver {
  pub async fn receive(&mut self) -> Option<Request> {
    let RequestReceiver(rx) = self;
    rx.recv().await
  }
}
