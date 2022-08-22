pub mod type_trait;
pub mod function;
pub mod signal;
pub mod process;

use crate::{repr::Pid, builtins::atoms::BuiltinAtoms};
use std::sync::{Arc, Mutex};

pub struct TaskContext {
  pub pid: Pid,
  pub mbox_rx: signal::SignalReceiver,
  pub builtin_atoms: Arc<Mutex<BuiltinAtoms>>,
}
