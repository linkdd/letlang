use logos::{Lexer, Logos};

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
#[logos(skip r"#.*\n?")]
pub enum Token<'source> {
  #[regex(r"[_a-zA-Z][_0-9a-zA-Z]*")]
  Ident(&'source str),

  #[regex(r#""([^"\\]|\\.)*""#, string)]
  String(String),

  #[regex(r"@(('(?:[^']|\\')+')|([_a-zA-Z][_a-zA-Z0-9]*))")]
  Atom(&'source str),

  #[token("true", |_| true)]
  #[token("false", |_| false)]
  Bool(bool),

  #[regex(r"0b_*[01][_01]*", int)]
  #[regex(r"0o_*[0-7][_0-7]*", int)]
  #[regex(r"[0-9][_0-9]*", int, priority = 2)]
  #[regex(r"0x_*[0-9a-fA-F][_0-9a-fA-F]*", int)]
  #[regex(r"((\d+\.?\d*)|(\.\d+))(([eE][+-]?)?\d+)?", float)]
  Number(f64),

  #[token("::")]
  ModSep,

  #[token("(")]
  ParenBegin,

  #[token(")")]
  ParenEnd,

  #[token("[")]
  BracketBegin,

  #[token("]")]
  BracketEnd,

  #[token("{")]
  BraceBegin,

  #[token("}")]
  BraceEnd,

  #[token("${")]
  DollarBraceBegin,

  #[token(",")]
  Comma,

  #[token(".")]
  Dot,

  #[token(":")]
  Colon,

  #[token(";")]
  Semicolon,

  #[token("->")]
  Arrow,

  #[token(":=")]
  Match,

  #[token("+")]
  Add,

  #[token("-")]
  Sub,

  #[token("*")]
  Mul,

  #[token("/")]
  Div,

  #[token("%")]
  Mod,

  #[token("**")]
  Pow,

  #[token("<")]
  LT,

  #[token("<=")]
  LTE,

  #[token("=")]
  Eq,

  #[token("!=")]
  Ne,

  #[token(">=")]
  GTE,

  #[token(">")]
  GT,

  #[token("!")]
  #[token("¬")]
  Negation,

  #[token("~")]
  BitNot,

  #[token("&")]
  BitAnd,

  #[token("|")]
  BitOr,

  #[token("^")]
  BitXor,

  #[token("<<")]
  BitLShift,

  #[token(">>")]
  BitRShift,

  #[token("not")]
  LogicNot,

  #[token("and")]
  LogicAnd,

  #[token("or")]
  LogicOr,

  #[token("<>")]
  StringConcat,

  #[token("++")]
  ListConcat,

  #[token("in")]
  ListContains,

  #[token("is")]
  TypeContains,

  #[token("|>")]
  Pipe,

  #[token("_")]
  PatternIgnore,

  #[token("...")]
  Ellipsis,

  #[token("module")]
  #[token("from")]
  #[token("import")]
  #[token("as")]
  #[token("let")]
  #[token("class")]
  #[token("func")]
  #[token("tailrec")]
  #[token("effect")]
  #[token("extern")]
  #[token("when")]
  #[token("with")]
  #[token("do")]
  #[token("intercept")]
  #[token("catch")]
  #[token("perform")]
  #[token("throw")]
  #[token("spawn")]
  #[token("receive")]
  #[token("after")]
  #[token("match")]
  #[token("cond")]
  #[token("else")]
  #[token("recurse")]
  #[token("final")]
  Keyword(&'source str),
}

fn int<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<f64, ()> {
  parse_int::parse::<i64>(lex.slice())
    .map(|i| i as f64)
    .map_err(|_| ())
}

fn float<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<f64, ()> {
  lex.slice().parse().map_err(|_| ())
}

fn string<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<String, ()> {
  snailquote::unescape(lex.slice()).map_err(|_| ())
}
