use std::path::PathBuf;
use std::ops::Range;

use ariadne::{Report, ReportKind, Label, Source};

#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation<'source> {
  pub filename: &'source PathBuf,
  pub code: &'source str,
  pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxErrorKind<'source> {
  InvalidToken(&'source str),
  UnexpectedToken(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxError<'source> {
  pub kind: SyntaxErrorKind<'source>,
  pub location: SourceLocation<'source>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OwnedSyntaxError {
  pub report: String,
}

pub type Result<'source, T> = std::result::Result<T, SyntaxError<'source>>;

impl<'source> std::fmt::Display for SyntaxErrorKind<'source> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SyntaxErrorKind::InvalidToken(tok) => write!(f, "Invalid token: {tok}"),
      SyntaxErrorKind::UnexpectedToken(tok) => write!(f, "Unexpected token: {tok}"),
    }
  }
}

impl<'source> std::fmt::Display for SyntaxError<'source> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let filename = self.location.filename.display().to_string();
    let code = self.location.code;
    let span = self.location.span.clone();

    let mut message: Vec<u8> = vec![];

    Report::build(ReportKind::Error, &filename, span.start)
      .with_code(match self.kind {
        SyntaxErrorKind::InvalidToken(..) => 101,
        SyntaxErrorKind::UnexpectedToken(..) => 102,
      })
      .with_label(Label::new((&filename, span)))
      .with_message(format!("Syntax Error: {}", self.kind))
      .finish()
      .write((&filename, Source::from(code)), &mut message)
      .expect("unable to write error report");

    let message = String::from_utf8_lossy(&message).into_owned();

    write!(f, "{}", message)
  }
}

impl<'source> std::fmt::Display for OwnedSyntaxError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.report)
  }
}

impl<'source> std::error::Error for SyntaxError<'source> {}
impl std::error::Error for OwnedSyntaxError {}

impl<'source> SyntaxError<'source> {
  pub fn to_owned(&self) -> OwnedSyntaxError {
    OwnedSyntaxError { report: format!("{}", self) }
  }
}
