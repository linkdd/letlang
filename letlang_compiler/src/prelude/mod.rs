use std::fmt;
use letlang_parser::ast::LocationInfo;

pub type CompilationResult<T> = std::result::Result<T, CompilationError>;

#[derive(Debug)]
pub struct CompilationError {
  pub(crate) message: String,
  located: bool
}

impl CompilationError {
  pub fn new(message: String) -> Self {
    Self { message, located: false }
  }

  pub fn new_located(location: &LocationInfo, message: String) -> Self {
    Self {
      message: format!(
        "[{};{}] {}",
        //location.filename.clone(),
        location.0,
        location.1,
        message
      ),
      located: true,
    }
  }

  pub fn is_located(&self) -> bool {
    self.located
  }
}

impl fmt::Display for CompilationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "CompilationError: {}", self.message)
  }
}

impl std::error::Error for CompilationError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    None
  }
}
