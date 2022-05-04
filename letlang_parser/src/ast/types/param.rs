use crate::ast::{Node, types::TypeRef};
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub struct TypeParam {
  pub name: String,
  pub constraint: Option<Node<TypeRef>>,
}

impl TypeParam {
  pub fn new(name: String, constraint: Option<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self { name, constraint })
  }
}
