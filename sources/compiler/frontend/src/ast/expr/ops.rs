#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
  LogicNot,
  BitNot,
  ArithmeticNegation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
  LogicAnd,
  LogicOr,
  BitAnd,
  BitOr,
  BitXor,
  BitLShift,
  BitRShift,
  ArithmeticAdd,
  ArithmeticSub,
  ArithmeticMul,
  ArithmeticDiv,
  ArithmeticMod,
  ArithmeticPow,
  CompareLT,
  CompareLTE,
  CompareEQ,
  CompareNE,
  CompareGTE,
  CompareGT,
  StringConcatenation,
  ListConcatenation,
  ListContains,
  ListNotContains,
}
