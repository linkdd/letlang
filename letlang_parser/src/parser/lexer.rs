use logos::{Logos, SpannedIter};

use crate::parser::{
  result::{ParseResult, ParseError},
  tokens::Token,
};

pub type Spanned<Tok, Loc> = ParseResult<(Loc, Tok, Loc)>;
pub struct Lexer<'input> {
  filename: String,
  input: &'input str,
  token_stream: SpannedIter<'input, Token>
}

impl<'input> Lexer<'input> {
  pub fn new(filename: &String, input: &'input str) -> Self {
    Self {
      filename: filename.clone(),
      input,
      token_stream: Token::lexer(input).spanned(),
    }
  }
}

impl<'input> Iterator for Lexer<'input> {
  type Item = Spanned<Token, usize>;

  fn next(&mut self) -> Option<Self::Item> {
    self.token_stream.next().map(|(token, span)| {
      match token {
        Token::Error => {
          Err(ParseError::new(
            format!(
              "{}:{}:{}: unexpected token {}",
              self.filename, span.start, span.end,
              self.input[span.clone()].to_string()
            )
          ))
        },
        _ => Ok((span.start, token, span.end))
      }
    })
  }
}
