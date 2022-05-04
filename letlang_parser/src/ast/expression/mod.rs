mod literal;
mod container;

pub use self::{
  literal::Literal,
  container::Container,
};

use crate::ast::{
  Node,
  funcs::{FunctionCall, EffectCall},
};

use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub enum Expression {
  Identifier(String),
  Literal(Node<Literal>),
  Container(Node<Container>),
  FunctionCall(Node<FunctionCall>),
  EffectCall(Node<EffectCall>),
  //ControlFlow
  //Coroutine
  //Assertion
  UnaryOperation { op: String, expr: Node<Expression> },
  BinaryOperation { lhs: Node<Expression>, op: String, rhs: Node<Expression> },
}

impl Expression {
  pub fn identifier(name: String) -> Box<Self> {
    Box::new(Self::Identifier(name))
  }

  pub fn literal(val: Node<Literal>) -> Box<Self> {
    Box::new(Self::Literal(val))
  }

  pub fn container(val: Node<Container>) -> Box<Self> {
    Box::new(Self::Container(val))
  }

  pub fn function_call(call: Node<FunctionCall>) -> Box<Self> {
    Box::new(Self::FunctionCall(call))
  }

  pub fn effect_call(call: Node<EffectCall>) -> Box<Self> {
    Box::new(Self::EffectCall(call))
  }

  pub fn unary_op(op: String, expr: Node<Expression>) -> Box<Self> {
    Box::new(Self::UnaryOperation { op, expr })
  }

  pub fn binary_op(lhs: Node<Expression>, op: String, rhs: Node<Expression>) -> Box<Self> {
    Box::new(Self::BinaryOperation { lhs, op, rhs })
  }
}
