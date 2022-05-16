mod literal;
mod container;
mod formula;
mod control_flow;

pub use self::{
  literal::Literal,
  container::Container,
  formula::{Formula, Proposition},
  control_flow::{FlowMatch, FlowMatchClause},
};

use crate::ast::{
  Node,
  funcs::{FunctionCall, EffectCall},
};

use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub enum Expression {
  Identifier { identifier: String },
  Literal(Node<Literal>),
  Container(Node<Container>),
  FunctionCall(Box<FunctionCall>),
  EffectCall(Box<EffectCall>),
  //ControlFlow
  FlowMatch(Box<FlowMatch>),
  //Coroutine
  //Assertion
  UnaryOperation { op: String, expr: Node<Expression> },
  BinaryOperation { lhs: Node<Expression>, op: String, rhs: Node<Expression> },
  #[serde(rename = "SolvableExpression")]
  Solvable(Node<Formula>),
}

impl Expression {
  pub fn identifier(name: String) -> Box<Self> {
    Box::new(Self::Identifier { identifier: name })
  }

  pub fn literal(val: Node<Literal>) -> Box<Self> {
    Box::new(Self::Literal(val))
  }

  pub fn container(val: Node<Container>) -> Box<Self> {
    Box::new(Self::Container(val))
  }

  pub fn function_call(call: Box<FunctionCall>) -> Box<Self> {
    Box::new(Self::FunctionCall(call))
  }

  pub fn effect_call(call: Box<EffectCall>) -> Box<Self> {
    Box::new(Self::EffectCall(call))
  }

  pub fn flow_match(block: Box<FlowMatch>) -> Box<Self> {
    Box::new(Self::FlowMatch(block))
  }

  pub fn unary_op(op: String, expr: Node<Expression>) -> Box<Self> {
    Box::new(Self::UnaryOperation { op, expr })
  }

  pub fn binary_op(lhs: Node<Expression>, op: String, rhs: Node<Expression>) -> Box<Self> {
    Box::new(Self::BinaryOperation { lhs, op, rhs })
  }

  pub fn solvable(formula: Node<Formula>) -> Box<Self> {
    Box::new(Self::Solvable(formula))
  }
}
