use crate::prelude::*;

mod token;
pub use self::token::Token;

use line_col::LineColLookup;
use logos::{Logos, Span};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct TokenLocation {
  filename: PathBuf,
  linecol: (usize, usize),
  offset: usize,
  token: Option<Token>,
}

impl std::fmt::Display for TokenLocation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let (line, col) = self.linecol;
    write!(f, "{}[{};{}]", self.filename.display(), line, col)
  }
}

pub struct TokenStream<'source> {
  filename: PathBuf,
  size: usize,
  tokens: Vec<(Token, Span)>,
  linecol_lookup: LineColLookup<'source>,
}

impl<'source> TokenStream<'source> {
  pub fn new(filename: PathBuf, input: &'source str) -> ParseResult<Self> {
    let token_stream = Self {
      filename,
      size: input.len(),
      tokens: Token::lexer(input).spanned().collect(),
      linecol_lookup: LineColLookup::new(input),
    };

    for (token, span) in token_stream.tokens.iter() {
      if let Token::Error = token {
        return Err(ParseError::new(format!("invalid token: {} {:?}", token, span)));
      }
    }

    Ok(token_stream)
  }
}

impl<'source> peg::Parse for TokenStream<'source> {
  type PositionRepr = TokenLocation;

  fn start<'input>(&'input self) -> usize {
    0
  }

  fn is_eof<'input>(&'input self, pos: usize) -> bool {
    pos >= self.tokens.len()
  }

  fn position_repr<'input>(&'input self, pos: usize) -> Self::PositionRepr {
    let (token, linecol, offset) = match self.tokens.get(pos) {
      Some((token, span)) => {
        (Some(token.clone()), self.linecol_lookup.get(span.start), span.start)
      },
      None => {
        (None, self.linecol_lookup.get(self.size), self.size)
      }
    };

    TokenLocation {
      filename: self.filename.clone(),
      linecol,
      offset,
      token
    }
  }
}

impl<'source, 'input> peg::ParseElem<'input> for TokenStream<'source> {
  type Element = &'input Token;

  fn parse_elem(&'input self, pos: usize) -> peg::RuleResult<Self::Element> {
    match self.tokens.get(pos) {
      Some((token, _)) => {
        peg::RuleResult::Matched(pos + 1, token)
      },
      None => {
        peg::RuleResult::Failed
      },
    }
  }
}

impl<'source> peg::ParseLiteral for TokenStream<'source> {
  fn parse_string_literal(&self, pos: usize, literal: &str) -> peg::RuleResult<()> {
    match self.tokens.get(pos) {
      Some((token, _)) => {
        match token {
          Token::Identifier(id) => {
            if id == literal {
              peg::RuleResult::Matched(pos + 1, ())
            }
            else {
              peg::RuleResult::Failed
            }
          },
          _ => {
            peg::RuleResult::Failed
          }
        }
      },
      None => {
        peg::RuleResult::Failed
      }
    }
  }
}

impl<'source, 'input> peg::ParseSlice<'input> for TokenStream<'source> {
  type Slice = Vec<&'input Token>;

  fn parse_slice(&'input self, begin_pos: usize, end_pos: usize) -> Self::Slice {
      let mut tokens = vec![];

      let mut pos = begin_pos;

      while pos < end_pos {
        let (token, _) = &self.tokens[pos];
        tokens.push(token);
        pos = pos + 1;
      }

      tokens
  }
}