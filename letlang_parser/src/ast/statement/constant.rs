use crate::ast::{Node, expression::Expression};
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub struct ConstDeclStatement {
  pub public: bool,
  pub symbol_name: String,
  pub value: Node<Expression>,
}
