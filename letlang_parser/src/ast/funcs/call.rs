use crate::ast::{
  Node,
  types::TypeRef,
  expression::Expression,
};

use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub struct FunctionCall {
  pub func_name: String,
  pub type_params: Vec<Node<TypeRef>>,
  pub call_params: Vec<Node<Expression>>,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub struct EffectCall {
  pub effect_name: String,
  pub type_params: Vec<Node<TypeRef>>,
  pub call_params: Vec<Node<Expression>>,
}

impl FunctionCall {
  pub fn new(
    func_name: String,
    type_params: Vec<Node<TypeRef>>,
    call_params: Vec<Node<Expression>>,
  ) -> Box<Self> {
    Box::new(Self { func_name, type_params, call_params })
  }
}

impl EffectCall {
  pub fn new(
    effect_name: String,
    type_params: Vec<Node<TypeRef>>,
    call_params: Vec<Node<Expression>>,
  ) -> Box<Self> {
    Box::new(Self { effect_name, type_params, call_params })
  }
}
