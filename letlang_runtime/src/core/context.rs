use std::sync::{Arc, Mutex};
use crate::{
  repr::Pid,
  core::{
    signal::SignalReceiver,
    atom_table::AtomTable,
  },
};

pub struct TaskContext {
  pub pid: Pid,
  pub mbox_rx: SignalReceiver,
  pub atom_table: Arc<Mutex<AtomTable>>,
}
