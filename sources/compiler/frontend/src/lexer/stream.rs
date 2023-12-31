use logos::Span;
use peg::Parse;

use crate::lexer::{Token, TokenLocation};

pub struct TokenStream<'source> {
  pub size: usize,
  pub tokens: Vec<(Token<'source>, Span)>,
}

impl<'source> TokenStream<'source> {
  pub fn span(&self, tok_begin: usize, tok_end: usize) -> (usize, usize) {
    let TokenLocation(begin, _) = self.position_repr(tok_begin);
    let TokenLocation(_, end) = self.position_repr(tok_end);
    (begin, end)
  }
}

impl<'source> peg::Parse for TokenStream<'source> {
  type PositionRepr = TokenLocation;

  fn start(&self) -> usize {
    0
  }

  fn is_eof(&self, pos: usize) -> bool {
    pos >= self.tokens.len()
  }

  fn position_repr(&self, pos: usize) -> Self::PositionRepr {
    match self.tokens.get(pos) {
      Some((_token, span)) => TokenLocation(span.start, span.end),
      None => TokenLocation(self.size, self.size),
    }
  }
}

impl<'source> peg::ParseElem<'source> for TokenStream<'source> {
  type Element = &'source Token<'source>;

  fn parse_elem(&'source self, pos: usize) -> peg::RuleResult<Self::Element> {
    match self.tokens.get(pos) {
      Some((token, _)) => peg::RuleResult::Matched(pos + 1, token),
      None => peg::RuleResult::Failed,
    }
  }
}

impl<'source> peg::ParseLiteral for TokenStream<'source> {
  fn parse_string_literal(&self, pos: usize, literal: &str) -> peg::RuleResult<()> {
    match self.tokens.get(pos) {
      Some((Token::Keyword(sym), _)) if sym == &literal => peg::RuleResult::Matched(pos + 1, ()),
      _ => peg::RuleResult::Failed,
    }
  }
}

impl<'source> peg::ParseSlice<'source> for TokenStream<'source> {
  type Slice = Vec<&'source Token<'source>>;

  fn parse_slice(&'source self, begin: usize, end: usize) -> Self::Slice {
    self.tokens[begin..end]
      .into_iter()
      .map(|(token, _)| token)
      .collect()
  }
}
