use std::{
  io::Error as IOError,
  str::Utf8Error,
};

pub type Result<T> = std::result::Result<T, CliError>;

pub struct CliError {
  pub message: String,
}

impl From<IOError> for CliError {
  fn from(value: IOError) -> Self {
    Self { message: format!("{value}") }
  }
}

impl From<Utf8Error> for CliError {
  fn from(value: Utf8Error) -> Self {
    Self { message: format!("{value}") }
  }
}

impl From<llfront::OwnedSyntaxError> for CliError {
  fn from(value: llfront::OwnedSyntaxError) -> Self {
    Self { message: value.report }
  }
}

impl From<Box<dyn std::error::Error>> for CliError {
  fn from(value: Box<dyn std::error::Error>) -> Self {
    Self { message: format!("{value}") }
  }
}
