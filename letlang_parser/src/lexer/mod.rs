mod token;
pub use self::token::Token;

use line_col::LineColLookup;
use logos::{Logos, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct TokenLocation {
  filename: String,
  linecol: (usize, usize),
  offset: usize
}

impl std::fmt::Display for TokenLocation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let (line, col) = self.linecol;
    write!(f, "{}[{};{}]", self.filename, line, col)
  }
}

pub struct TokenStream<'source> {
  filename: &'source str,
  size: usize,
  tokens: Vec<(Token, Span)>,
  linecol_lookup: LineColLookup<'source>,
}

impl<'source> TokenStream<'source> {
  pub fn new(filename: &'source str, input: &'source str) -> Self {
    Self {
      filename,
      size: input.len(),
      tokens: Token::lexer(input).spanned().collect(),
      linecol_lookup: LineColLookup::new(input),
    }
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
    let (linecol, offset) = match self.tokens.get(pos) {
      Some((_, span)) => {
        (self.linecol_lookup.get(span.start), span.start)
      },
      None => {
        (self.linecol_lookup.get(self.size), self.size)
      }
    };

    TokenLocation {
      filename: self.filename.to_string(),
      linecol,
      offset
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