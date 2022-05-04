use crate::ast::{
  Node,
  types::TypeParam,
  class::{ConsParam, Formula},
};

use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub struct ClassDeclStatement {
  pub symbol_name: String,
  pub type_params: Vec<Node<TypeParam>>,
  pub cons_params: Vec<Node<ConsParam>>,
  pub constraints: Node<Formula>,
}
