use crate::{
  Node,
  types::TypeRef,
  expression::{Expression, Pattern},
};

#[derive(Debug, Clone, PartialEq)]
pub struct GenericResolve {
  pub symbol: Node<Expression>,
  pub type_params: Vec<Node<TypeRef>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberAccess {
  pub lhs: Node<Expression>,
  pub rhs: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeCheck {
  pub lhs: Node<Expression>,
  pub rhs: Node<TypeRef>,
  pub not: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOperation {
  pub op: &'static str,
  pub expr: Node<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperation {
  pub lhs: Node<Expression>,
  pub op: &'static str,
  pub rhs: Node<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PatternMatch {
  pub lhs: Node<Pattern>,
  pub rhs: Node<Expression>,
}

impl Expression {
  pub fn generic_resolve(symbol: Node<Expression>, type_params: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::GenericResolve(GenericResolve { symbol, type_params }))
  }

  pub fn member_access(lhs: Node<Expression>, rhs: String) -> Box<Self> {
    Box::new(Self::MemberAccess(MemberAccess { lhs, rhs }))
  }

  pub fn type_check(lhs: Node<Expression>, rhs: Node<TypeRef>, not: bool) -> Box<Self> {
    Box::new(Self::TypeCheck(TypeCheck { lhs, rhs, not }))
  }

  pub fn unary_op(op: &'static str, expr: Node<Expression>) -> Box<Self> {
    Box::new(Self::UnaryOperation(UnaryOperation { op, expr }))
  }

  pub fn binary_op(op: &'static str, lhs: Node<Expression>, rhs: Node<Expression>) -> Box<Self> {
    Box::new(Self::BinaryOperation(BinaryOperation { lhs, op, rhs }))
  }

  pub fn pattern_match(lhs: Node<Pattern>, rhs: Node<Expression>) -> Box<Self> {
    Box::new(Self::PatternMatch(PatternMatch { lhs, rhs }))
  }
}
