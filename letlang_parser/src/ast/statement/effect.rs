use crate::ast::{
  Node,
  types::{TypeParam, TypeRef},
  funcs::CallParam,
};

use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub struct EffectDeclStatement {
  pub symbol_name: String,
  pub type_params: Vec<Node<TypeParam>>,
  pub call_params: Vec<Node<CallParam>>,
  pub return_type: Node<TypeRef>,
}
