#[derive(Debug)]
pub enum RuntimeError {
  NodeServerFailed,
  ChannelRecvFailed,
  ChannelSendFailed,
  ProcessNotFound,
  ProcessCrashed,
  EffectNotImplemented,
  UncaughtException,
  Other(&'static str)
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

unsafe impl Sync for RuntimeError {}
unsafe impl Send for RuntimeError {}
