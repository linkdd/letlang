use crate::ast::{
  Node,
  types::{TypeParam, TypeRef},
  funcs::CallParam,
  expression::Expression,
};

use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub struct FuncDeclStatement {
  pub symbol_name: String,
  pub type_params: Vec<Node<TypeParam>>,
  pub call_params: Vec<Node<CallParam>>,
  pub return_type: Node<TypeRef>,
  pub body: Vec<Node<Expression>>,
}
