use crate::prelude::*;
use crate::core::{signal::Signal, function::Function};
use crate::repr::Pid;

#[derive(Debug)]
pub enum Command {
  Spawn(Box<dyn Function>),
  SendTo(Pid, Signal),
  Link(Pid, Pid),
  Unlink(Pid, Pid),
  Exited(Pid, Result<()>),
}
