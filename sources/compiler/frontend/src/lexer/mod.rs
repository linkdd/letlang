use std::path::PathBuf;
use logos::{Logos, Span};

use crate::prelude::*;

mod location;
mod token;
mod stream;

pub use self::{
  location::*,
  token::*,
  stream::*,
};

pub fn lex<'source>(
  filename: &'source PathBuf,
  code: &'source str,
) -> Result<'source, TokenStream<'source>> {
  let tokens = Token::lexer(code)
    .spanned()
    .map(|(res, span)| match res {
      Ok(token) => Ok((token, span)),
      Err(_) => Err(SyntaxError {
        kind: SyntaxErrorKind::InvalidToken(&code[span.clone()]),
        location: SourceLocation {
          filename,
          code,
          span: span.clone(),
        },
      }),
    })
    .collect::<Result<Vec<(Token<'source>, Span)>>>()?;

  Ok(TokenStream {
    size: code.len(),
    tokens,
  })
}
