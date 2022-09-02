use crate::repr::Pid;
use crate::core::signal;

pub type Process = (Pid, Vec<Pid>, signal::SignalSender, signal::SignalReceiver);

pub fn new() -> Process {
  let pid = Pid::new();
  let links = vec![];
  let (tx, rx) = signal::channel();
  (pid, links, tx, rx)
}
