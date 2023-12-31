use ariadne::{Report, ReportKind, Label, Source};
use llfront::SourceLocation;

#[derive(Debug, Clone, PartialEq)]
pub enum CompilationErrorKind {
  ClauseArrity { expected: usize, got: usize }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompilationError<'source> {
  pub location: SourceLocation<'source>,
  pub kind: CompilationErrorKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OwnedCompilationError {
  pub report: String,
}

pub type Result<'source, T> = std::result::Result<T, CompilationError<'source>>;

impl std::fmt::Display for CompilationErrorKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ClauseArrity { expected, got } => {
        write!(f, "Invalid clause arity, expected {expected} arguments, got {got}")
      }
    }
  }
}

impl<'source> std::fmt::Display for CompilationError<'source> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let filename = self.location.filename.display().to_string();
    let code = self.location.code;
    let span = self.location.span.clone();

    let mut message: Vec<u8> = vec![];

    Report::build(ReportKind::Error, &filename, span.start)
      .with_code(200)
      .with_label(Label::new((&filename, span)))
      .with_message(format!("Compilation Error: {}", self.kind))
      .finish()
      .write((&filename, Source::from(code)), &mut message)
      .expect("unable to write error report");

    let message = String::from_utf8_lossy(&message).into_owned();

    write!(f, "{}", message)
  }
}

impl<'source> std::fmt::Display for OwnedCompilationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.report)
  }
}

impl<'source> std::error::Error for CompilationError<'source> {}
impl std::error::Error for OwnedCompilationError {}

impl<'source> CompilationError<'source> {
  pub fn to_owned(&self) -> OwnedCompilationError {
    OwnedCompilationError { report: format!("{}", self) }
  }
}
