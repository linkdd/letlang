use std::collections::HashMap;
use crate::ast::{Node, types::TypeRef};
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub enum Container {
  Tuple(Vec<Node<TypeRef>>),
  List(Node<TypeRef>),
  Set(Node<TypeRef>),
  Map(HashMap<String, Node<TypeRef>>),
}

impl Container {
  pub fn tuple(types: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::Tuple(types))
  }

  pub fn list(items_type: Node<TypeRef>) -> Box<Self> {
    Box::new(Self::List(items_type))
  }

  pub fn set(items_type: Node<TypeRef>) -> Box<Self> {
    Box::new(Self::Set(items_type))
  }

  pub fn map(items: HashMap<String, Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::Map(items))
  }
}
