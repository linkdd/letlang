pub mod prelude;
pub mod lexer;
pub mod parser;
pub mod ast;

use std::{
  io::Error as IOError,
  fs,
  path::{Path, PathBuf},
};

pub use self::{
  prelude::{SourceLocation, SyntaxError, OwnedSyntaxError},
  ast::AST,
};

pub struct SourceFile {
  filename: PathBuf,
  code: String,
}

impl SourceFile {
  pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<Self, IOError> {
    let filename = filename.as_ref().to_path_buf();
    let code = fs::read_to_string(&filename)?;
    Ok(Self { filename, code })
  }
}

impl<'source> TryFrom<&'source SourceFile> for AST<SourceLocation<'source>> {
  type Error = SyntaxError<'source>;

  fn try_from(value: &'source SourceFile) -> Result<Self, Self::Error> {
    let tokens = lexer::lex(&value.filename, &value.code)?;
    let ast = parser::parse(&value.filename, &value.code, tokens)?;
    Ok(ast)
  }
}
