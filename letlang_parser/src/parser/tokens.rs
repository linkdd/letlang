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

  #[regex(r"[1-9][_1-9]*", |lex| {
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

  #[regex(r"@(('(?:[^']|\\')+')|([_a-zA-Z][_a-zA-Z0-9]*))")]
  Atom,

  // statement keywords
  #[token("module")]
  StatementModule,

  #[token("import")]
  StatementImport,

  #[token("as")]
  StatementImportAlias,

  #[token("const")]
  StatementConst,

  #[token("class")]
  StatementClass,

  #[token("effect")]
  StatementEffect,

  #[token("func")]
  StatementFunction,

  #[token("pub")]
  Public,

  // block keywords
  #[token("check")]
  BlockCheck,

  #[token("do")]
  BlockDo,

  #[token("catch")]
  BlockCatchClause,

  #[token("intercept")]
  BlockEffectClause,

  #[token("finally")]
  BlockFinallyClause,

  // expression keywords
  #[token("thereis")]
  QuantifierThereIs,

  #[token("forall")]
  QuantifierForAll,

  #[token("let")]
  ExprLet,

  #[token("throw")]
  ExprThrow,

  #[token("perform")]
  ExprPerform,

  #[token("resume")]
  ExprResume,

  #[token("coro")]
  ExprCoro,

  #[token("join")]
  ExprJoin,

  // control flow keywords
  #[token("if")]
  FlowIf,

  #[token("else")]
  FlowElse,

  #[token("match")]
  FlowMatch,

  #[token("when")]
  FlowWhen,

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
  StatementSeparator,

  #[token(",")]
  ExprSeparator,

  #[token(":")]
  PairSeparator,

  #[token("->")]
  Annotation,

  #[token("!")]
  Negation,

  // operators
  #[token(":=")]
  OperatorAssign,

  #[token(".")]
  OperatorAccess,

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

  #[token("==>")]
  OperatorLogicalImply,

  #[token("<==>")]
  OperatorLogicalBicondition,

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

  #[token("|>>")]
  OperatorStreamRead,

  #[token("|<<")]
  OperatorStreamWrite,

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
