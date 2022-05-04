use crate::ast::{
  Node,
  expression::Literal,
  types::container::Container,
};

use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub enum TypeRef {
  Value(Node<Literal>),
  Container(Node<Container>),
  TypeName(String),
  OneOf(Vec<Node<TypeRef>>),
  AllOf(Vec<Node<TypeRef>>),
  Not(Node<TypeRef>),
}

impl TypeRef {
  pub fn value(val: Node<Literal>) -> Box<Self> {
    Box::new(Self::Value(val))
  }

  pub fn container(val: Node<Container>) -> Box<Self> {
    Box::new(Self::Container(val))
  }

  pub fn type_name(name: String) -> Box<Self> {
    Box::new(Self::TypeName(name))
  }

  pub fn one_of(types: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::OneOf(types))
  }

  pub fn all_of(types: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::AllOf(types))
  }

  pub fn not(type_ref: Node<TypeRef>) -> Box<Self> {
    Box::new(Self::Not(type_ref))
  }
}
