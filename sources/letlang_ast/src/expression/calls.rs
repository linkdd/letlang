use crate::{
  Node,
  expression::{Symbol, Expression},
};

#[derive(Debug, Clone, PartialEq)]
pub struct EffectCall {
  pub effect_name: Symbol,
  pub params: Vec<Node<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
  pub func: Node<Expression>,
  pub params: Vec<Node<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpawnCall {
  pub func: Node<Expression>,
  pub params: Vec<Node<Expression>>,
}

impl Expression {
  pub fn effect_call(effect_name: Vec<String>, params: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::EffectCall(EffectCall {
      effect_name: Symbol(effect_name),
      params
    }))
  }

  pub fn function_call(func: Node<Expression>, params: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::FunctionCall(FunctionCall { func, params }))
  }

  pub fn spawn_call(func: Node<Expression>, params: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::SpawnCall(SpawnCall { func, params }))
  }
}
