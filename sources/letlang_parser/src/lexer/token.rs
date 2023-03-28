use std::fmt;
use logos::Logos;
use snailquote::unescape;


#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token {
  // literals
  #[regex(r"[_a-zA-Z][_0-9a-zA-Z]*", |lex| lex.slice().parse())]
  Identifier(String),

  #[token("true")]
  True,

  #[token("false")]
  False,

  #[regex(r"0b_*[01][_01]*", |lex| {
    parse_int::parse::<i64>(lex.slice()).map(|i| i as f64)
  })]
  IntegerBase2(f64),

  #[regex(r"0o_*[0-7][_0-7]*", |lex| {
    parse_int::parse::<i64>(lex.slice()).map(|i| i as f64)
  })]
  IntegerBase8(f64),

  #[regex(r"[1-9][_0-9]*", |lex| {
    parse_int::parse::<i64>(lex.slice()).map(|i| i as f64)
  }, priority = 2)]
  IntegerBase10(f64),

  #[regex(r"0x_*[0-9a-fA-F][_0-9a-fA-F]*", |lex| {
    parse_int::parse::<i64>(lex.slice()).map(|i| i as f64)
  })]
  IntegerBase16(f64),

  #[regex(r"((\d+\.?\d*)|(\.\d+))(([eE][+-]?)?\d+)?", |lex| {
    lex.slice().parse()
  })]
  Float(f64),

  #[regex("\"(?:[^\"]|\\\\\")*\"", |lex| {
    unescape(lex.slice())
  })]
  String(String),

  #[regex(r"@(('(?:[^']|\\')+')|([_a-zA-Z][_a-zA-Z0-9]*))", |lex| {
    lex.slice().parse()
  })]
  Atom(String),

  // keywords
  #[token("module")]
  KeywordModule,

  #[token("import")]
  KeywordImport,

  #[token("as")]
  KeywordAs,

  #[token("class")]
  KeywordClass,

  #[token("effect")]
  KeywordEffect,

  #[token("func")]
  KeywordFunc,

  #[token("tailrec")]
  KeywordTailRec,

  #[token("final")]
  KeywordFinal,

  #[token("recurse")]
  KeywordRecurse,

  #[token("pub")]
  KeywordPub,

  #[token("do")]
  KeywordDo,

  #[token("catch")]
  KeywordCatch,

  #[token("intercept")]
  KeywordIntercept,

  #[token("let")]
  KeywordLet,

  #[token("throw")]
  KeywordThrow,

  #[token("perform")]
  KeywordPerform,

  #[token("spawn")]
  KeywordSpawn,

  #[token("receive")]
  KeywordReceive,

  #[token("after")]
  KeywordAfter,

  #[token("cond")]
  KeywordCond,

  #[token("else")]
  KeywordElse,

  #[token("match")]
  KeywordMatch,

  #[token("=>")]
  FatArrow,

  // bracket symbols
  #[token("{")]
  CurlyBracketBegin,

  #[token("}")]
  CurlyBracketEnd,

  #[token("[")]
  BracketBegin,

  #[token("]")]
  BracketEnd,

  #[token("(")]
  ParenthesisBegin,

  #[token(")")]
  ParenthesisEnd,

  // symbols
  #[token(";")]
  Semicolon,

  #[token(",")]
  Comma,

  #[token(":")]
  Colon,

  #[token("::")]
  DoubleColon,

  #[token("->")]
  Arrow,

  #[token("!")]
  Negation,

  #[token("$")]
  Dollar,

  // operators
  #[token(":=")]
  OperatorMatch,

  #[token(".")]
  OperatorDot,

  #[token("<")]
  OperatorCmpLT,

  #[token("<=")]
  OperatorCmpLTE,

  #[token("=")]
  OperatorCmpEQ,

  #[token("!=")]
  OperatorCmpNE,

  #[token(">=")]
  OperatorCmpGTE,

  #[token(">")]
  OperatorCmpGT,

  #[token("not")]
  OperatorLogicalNot,

  #[token("and")]
  OperatorLogicalAnd,

  #[token("or")]
  OperatorLogicalOr,

  #[token("+")]
  OperatorMathAdd,

  #[token("-")]
  OperatorMathSub,

  #[token("*")]
  OperatorMathMul,

  #[token("/")]
  OperatorMathDiv,

  #[token("%")]
  OperatorMathMod,

  #[token("**")]
  OperatorMathPow,

  #[token("&")]
  OperatorBinAnd,

  #[token("|")]
  OperatorBinOr,

  #[token("^")]
  OperatorBinXor,

  #[token("~")]
  OperatorBinNot,

  #[token("<<")]
  OperatorBinLShift,

  #[token(">>")]
  OperatorBinRShift,

  #[token("<>")]
  OperatorStringConcat,

  #[token("|>")]
  OperatorPipeline,

  #[token("in")]
  OperatorIn,

  #[token("is")]
  OperatorIs,

  // comments and whitespaces
  #[regex(r"#.*\n?", logos::skip)]
  #[regex(r"[ \t\r\n\f]+", logos::skip)]
  #[error]
  Error,
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
