use std::path::PathBuf;
use crate::{lexer::TokenStream, SourceLocation};

pub struct SourceMapLookup<'source, 'tokens> {
  pub filename: &'source PathBuf,
  pub code: &'source str,
  pub token_stream: &'tokens TokenStream<'source>,
}

impl<'source, 'tokens> SourceMapLookup<'source, 'tokens> {
  pub fn token_span(&self, tok_begin: usize, tok_end: usize) -> SourceLocation<'source> {
    let tok_end = tok_end.saturating_sub(1);
    let (char_begin, char_end) = self.token_stream.span(tok_begin, tok_end);

    SourceLocation {
      filename: self.filename,
      code: self.code,
      span: char_begin..char_end,
    }
  }
}
