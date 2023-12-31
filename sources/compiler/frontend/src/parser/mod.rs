mod sourcemap;
mod grammar;

use std::path::PathBuf;
use crate::{
  ast::AST,
  lexer::{TokenStream, TokenLocation},
  prelude::*,
};
use self::{
  sourcemap::SourceMapLookup,
  grammar::unit_parser,
};

pub fn parse<'source>(
  filename: &'source PathBuf,
  code: &'source str,
  token_stream: TokenStream<'source>
) -> Result<'source, AST<SourceLocation<'source>>> {
  let srcmap = SourceMapLookup {
    filename,
    code,
    token_stream: &token_stream,
  };

  let ast = unit_parser::unit(&token_stream, &srcmap)
    .map_err(|err| {
      let TokenLocation(begin, end) = err.location;

      SyntaxError {
        location: SourceLocation {
          filename,
          code,
          span: begin..end,
        },
        kind: SyntaxErrorKind::UnexpectedToken(err.expected.to_string()),
      }
    })?;

  Ok(ast)
}
