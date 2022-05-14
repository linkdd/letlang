use crate::ast::{Node, types::TypeRef};
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub enum Container {
  #[serde(rename = "TupleType")]
  Tuple(Vec<Node<TypeRef>>),
  #[serde(rename = "StructType")]
  Struct(Vec<(String, Node<TypeRef>)>),
}

impl Container {
  pub fn tuple(types: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::Tuple(types))
  }

  pub fn structure(items: Vec<(String, Node<TypeRef>)>) -> Box<Self> {
    Box::new(Self::Struct(items))
  }
}
