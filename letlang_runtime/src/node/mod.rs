mod command;
mod request;
mod response;
mod task;

pub use self::{
  command::Command,
  request::{Request, RequestSender, RequestReceiver},
};

use crate::{
  prelude::*,
  repr::{Pid, Value},
  core::{
    signal,
    signal::Signal,
    process,
    function::Function,
    TaskContext,
  },
  builtins::atoms::BuiltinAtoms,
};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Node {
  builtin_atoms: Arc<Mutex<BuiltinAtoms>>,
  process_registry: HashMap<Pid, (Vec<Pid>, signal::SignalSender)>,
  request_tx: RequestSender,
  request_rx: RequestReceiver,
}

impl Node {
  pub fn new(builtin_atoms: BuiltinAtoms) -> Self {
    let (tx, rx) = request::channel();

    Self {
      builtin_atoms: Arc::new(Mutex::new(builtin_atoms)),
      process_registry: HashMap::new(),
      request_tx: tx,
      request_rx: rx,
    }
  }

  pub async fn run(mut self, main: Box<dyn Function>) -> Result<()> {
    Request::publish(Command::Spawn(main), &mut self.request_tx).await?;

    let request_server_handle = tokio::spawn(async move {
      while let Some(req) = self.request_rx.receive().await {
        let res = self.process_command(req.command).await;
        req.respond_to.send(res).unwrap();

        if self.process_registry.is_empty() {
          break;
        }
      }
    });

    request_server_handle.await.map_err(|_err| {
      RuntimeError::NodeServerFailed
    })?;

    Ok(())
  }

  pub async fn process_command(&mut self, command: Command) -> Result<()> {
    match command {
      Command::Spawn(func) => {
        self.spawn_process(func)
      },
      Command::Link(source, target) => {
        self.link_processes(source, target)
      },
      Command::Unlink(source, target) => {
        self.unlink_processes(source, target)
      },
      Command::SendTo(target, signal) => {
        self.send_signal(target, signal).await
      },
      Command::Exited(proc_id, result) => {
        self.process_exited(proc_id, result).await
      }
    }
  }

  fn spawn_process(&mut self, func: Box<dyn Function>) -> Result<()> {
    let mut node_req_tx = self.request_tx.clone();

    let (pid, links, mbox_tx, mbox_rx) = process::new();
    self.process_registry.insert(pid.clone(), (links, mbox_tx));

    let builtin_atoms = self.builtin_atoms.clone();

    tokio::spawn(async move {
      let proc_id = pid.clone();

      let process_handle = tokio::spawn(async move {
        let context = TaskContext {
          pid: proc_id,
          mbox_rx,
          builtin_atoms
        };

        task::run(context, func).await.unwrap();
      });

      let res = process_handle.await.map_err(|_err| {
        RuntimeError::ProcessCrashed
      });

      Request::publish(Command::Exited(pid.clone(), res), &mut node_req_tx).await.unwrap();
    });

    Ok(())
  }

  fn link_processes(&mut self, source: Pid, target: Pid) -> Result<()> {
    match self.process_registry.get_mut(&target) {
      None => Err(RuntimeError::ProcessNotFound),
      Some((links, _)) => {
        links.push(source);
        links.dedup();
        Ok(())
      }
    }
  }

  fn unlink_processes(&mut self, source: Pid, target: Pid) -> Result<()> {
    match self.process_registry.get_mut(&target) {
      None => Err(RuntimeError::ProcessNotFound),
      Some((links, _)) => {
        links.retain(|pid| *pid != source);
        Ok(())
      }
    }
  }

  async fn send_signal(&mut self, target: Pid, signal: Signal) -> Result<()> {
    match self.process_registry.get_mut(&target) {
      None => Err(RuntimeError::ProcessNotFound),
      Some((_, mbox_tx)) => {
        mbox_tx.send(signal).await
      }
    }
  }

  async fn process_exited(&mut self, proc_id: Pid, result: Result<()>) -> Result<()> {
    let reason = match result {
      Ok(_) => {
        Value::Atom(self.builtin_atoms.lock().unwrap().get("@normal"))
      },
      Err(err) => {
        Value::Tuple(Box::new([
          Value::Atom(self.builtin_atoms.lock().unwrap().get("@error")),
          Value::String(format!("{:?}", err))
        ]))
      }
    };

    let signal = Signal::Exited(proc_id.clone(), reason);

    let (links, _) = match self.process_registry.remove(&proc_id) {
      None => Err(RuntimeError::ProcessNotFound),
      Some(proc_info) => Ok(proc_info),
    }?;

    for link in links {
      match self.process_registry.get_mut(&link) {
        None => {},
        Some((_, mbox_tx)) => {
          mbox_tx.send(signal.clone()).await?;
        }
      }
    }

    Ok(())
  }
}
