use crate::ast::{
  Node,
  types::TypeParam,
  class::ConsParam,
  expression::Expression,
};

use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub struct ClassDeclStatement {
  pub public: bool,
  pub symbol_name: String,
  pub type_params: Vec<Node<TypeParam>>,
  pub cons_param: Node<ConsParam>,
  pub constraints: Vec<Node<Expression>>,
}
