use std::fmt;

pub type Result<T> = std::result::Result<T, LetlangError>;

#[derive(Debug, Clone)]
pub enum LetlangError {
  NotImplementedError { message: String },
  ImportError { module: String, message: String },
  ExportError { symbol_name: String, message: String },
  TypeError { expected: String, got: String },
  AttributeError { key: String, message: String },
}

impl fmt::Display for LetlangError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      LetlangError::NotImplementedError { message } => {
        write!(f, "NotImplementedError: {}", message)
      }
      LetlangError::ImportError { module, message } => {
        write!(f, "ImportError: {} while loading module {}", message, module)
      }
      LetlangError::ExportError { symbol_name, message } => {
        write!(f, "ExportError: {} while exporting symbol {}", message, symbol_name)
      }
      LetlangError::TypeError { expected, got } => {
        write!(f, "TypeError: expected {}, got {}", expected, got)
      }
      LetlangError::AttributeError { key, message } => {
        write!(f, "AttributeError: {} while accessing key {}", message, key)
      }
    }
  }
}
