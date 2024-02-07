use crate::{
  prelude::*,
  data::{
    LLPid,
    LLValue,
    LLAtom,
  },
  concurrency::{
    signal::LLSignal,
    process::{
      LLProcess,
      LLProcessHandle,
    },
  },
  thread::LLTask,
  traits::{
    LLFunctionInstance,
    LLClassInstance,
  },
};

use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;

mod command;
mod request;
mod response;

use self::{
  command::*,
  request::*,
  response::*,
};

pub struct LLProcessGroup {
  group_id: u64,
  last_local_id: AtomicU64,
  process_registry: HashMap<LLPid, LLProcessHandle>,
  request_tx: RequestSender,
  request_rx: RequestReceiver,
}


impl LLProcessGroup {
  pub fn new(group_id: u64) -> Self {
    let (tx, rx) = RequestChannel::new();
    Self {
      group_id,
      last_local_id: AtomicU64::new(0),
      process_registry: HashMap::new(),
      request_tx: tx,
      request_rx: rx,
    }
  }

  fn make_pid(&self) -> LLPid {
    let local_id = self.last_local_id.fetch_add(1, Ordering::SeqCst);
    LLPid::new(self.group_id, local_id)
  }

  pub fn run(self, main: LLFunctionInstance) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
      self.serve(main).await
    })
  }

  async fn serve(mut self, main: LLFunctionInstance) -> Result<()> {
    let mut req_tx = self.request_tx.clone();

    let server_handle = tokio::spawn(async move {
      while let Some(req) = self.request_rx.receive().await {
        let res = self.handle_command(req.command).await;
        req.respond_to.send(res).unwrap();

        if self.process_registry.is_empty() {
          break;
        }
      }
    });

    Command::Spawn(vec![], main).send(&mut req_tx).await?;

    server_handle.await.map_err(|_| RuntimeError::ProcessGroupCrashed)?;

    Ok(())
  }

  async fn handle_command(&mut self, cmd: Command) -> Result<()> {
    match cmd {
      Command::Spawn(type_params, func) => {
        self.handle_command_spawn(type_params, func)
      },
      Command::SendTo(pid, signal) => {
        self.handle_command_send_to(pid, signal).await
      },
      Command::Link(source, target) => {
        self.handle_command_link(source, target)
      },
      Command::Unlink(source, target) => {
        self.handle_command_unlink(source, target)
      },
      Command::Exited(pid, res) => {
        self.handle_command_exited(pid, res).await
      },
    }
  }

  fn handle_command_spawn(
    &mut self,
    type_params: Vec<LLClassInstance>,
    func: LLFunctionInstance,
  ) -> Result<()> {
    let mut group_req_tx = self.request_tx.clone();

    let pid = self.make_pid();
    let (ctx, handle) = LLProcess::new(pid.clone());
    self.process_registry.insert(pid.clone(), handle);

    tokio::spawn(async move {
      let task_handle = tokio::spawn(async move {
        let task = LLTask {
          context: ctx,
          type_params,
          func,
        };
        task.run().await.unwrap();
      });

      let res = task_handle.await.map_err(|_| RuntimeError::ProcessCrashed);
      Command::Exited(pid.clone(), res).send(&mut group_req_tx).await.unwrap();
    });

    Ok(())
  }

  async fn handle_command_send_to(
    &mut self,
    target: LLPid,
    signal: LLSignal,
  ) -> Result<()> {
    match self.process_registry.get_mut(&target) {
      None => Err(RuntimeError::ProcessNotFound),
      Some(handle) => handle.mbox_tx.send(signal).await,
    }
  }

  fn handle_command_link(
    &mut self,
    source: LLPid,
    target: LLPid,
  ) -> Result<()> {
    match self.process_registry.get_mut(&target) {
      None => Err(RuntimeError::ProcessNotFound),
      Some(handle) => {
        handle.links.push(source);
        Ok(())
      },
    }
  }

  fn handle_command_unlink(
    &mut self,
    source: LLPid,
    target: LLPid,
  ) -> Result<()> {
    match self.process_registry.get_mut(&target) {
      None => Err(RuntimeError::ProcessNotFound),
      Some(handle) => {
        handle.links.retain(|pid| *pid != source);
        Ok(())
      },
    }
  }

  async fn handle_command_exited(
    &mut self,
    pid: LLPid,
    res: Result<()>,
  ) -> Result<()> {
    let reason = match res {
      Ok(_) => LLValue::Atom(LLAtom::new("@normal")),
      Err(err) => {
        LLValue::Tuple(Box::new([
          LLValue::Atom(LLAtom::new("@error")),
          LLValue::String(format!("{}", err)),
        ]))
      },
    };

    let signal = LLSignal::Exited(pid.clone(), reason);

    let handle = self.process_registry.remove(&pid)
      .ok_or(RuntimeError::ProcessNotFound)?;

    for link in handle.links.iter() {
      if let Some(handle) = self.process_registry.get_mut(&link) {
        handle.mbox_tx.send(signal.clone()).await?;
      }
    }

    Ok(())
  }
}
