use crate::prelude::*;
use super::{
  command::Command,
  response::ResponseSender,
};

use tokio::sync::mpsc;

pub struct Request {
  pub respond_to: ResponseSender,
  pub command: Command,
}

pub struct RequestChannel;

#[derive(Clone)]
pub struct RequestSender(mpsc::Sender<Request>);

pub struct RequestReceiver(mpsc::Receiver<Request>);

impl RequestChannel {
  pub fn new() -> (RequestSender, RequestReceiver) {
    let (tx, rx) = mpsc::channel(32);
    (RequestSender(tx), RequestReceiver(rx))
  }
}

impl RequestSender {
  pub async fn send(&self, req: Request) -> Result<()> {
    let Self(tx) = self;
    tx.send(req).await?;
    Ok(())
  }
}

impl RequestReceiver {
  pub async fn receive(&mut self) -> Option<Request> {
    let Self(rx) = self;
    rx.recv().await
  }
}
