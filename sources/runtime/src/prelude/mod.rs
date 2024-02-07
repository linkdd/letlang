use tokio::sync::{oneshot, mpsc};

#[derive(Debug)]
pub enum RuntimeError {
  ChannelReceiveError,
  ChannelSendError,
  ProcessGroupCrashed,
  ProcessCrashed,
  ProcessNotFound,
  UncaughtException,
  IOError(std::io::Error),
}

pub type Result<T> = std::result::Result<T, RuntimeError>;

unsafe impl Sync for RuntimeError {}
unsafe impl Send for RuntimeError {}

impl std::fmt::Display for RuntimeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ChannelReceiveError => write!(f, "Channel receive error"),
      Self::ChannelSendError => write!(f, "Channel send error"),
      Self::ProcessGroupCrashed => write!(f, "Process group crashed"),
      Self::ProcessCrashed => write!(f, "Process crashed"),
      Self::ProcessNotFound => write!(f, "Process not found"),
      Self::UncaughtException => write!(f, "Uncaught exception"),
      Self::IOError(e) => write!(f, "IO error: {}", e),
    }
  }
}

impl std::error::Error for RuntimeError {}

impl<T> From<mpsc::error::SendError<T>> for RuntimeError {
  fn from(_: mpsc::error::SendError<T>) -> Self {
    Self::ChannelSendError
  }
}

impl From<oneshot::error::RecvError> for RuntimeError {
  fn from(_: oneshot::error::RecvError) -> Self {
    Self::ChannelReceiveError
  }
}

impl From<std::io::Error> for RuntimeError {
  fn from(e: std::io::Error) -> Self {
    Self::IOError(e)
  }
}
