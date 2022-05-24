use crate::ast::{
  Node,
  expression::Literal,
  types::container::Container,
};

use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub enum TypeRef {
  #[serde(rename = "ValueType")]
  Value(Node<Literal>),
  #[serde(rename = "ContainerType")]
  Container(Node<Container>),
  TypeName { name: String, type_params: Vec<Node<TypeRef>> },
  #[serde(rename = "OneOfType")]
  OneOf { typerefs: Vec<Node<TypeRef>> },
  #[serde(rename = "AllOfType")]
  AllOf { typerefs: Vec<Node<TypeRef>> },
  #[serde(rename = "NotType")]
  Not { typeref: Node<TypeRef> },
}

impl TypeRef {
  pub fn value(val: Node<Literal>) -> Box<Self> {
    Box::new(Self::Value(val))
  }

  pub fn container(val: Node<Container>) -> Box<Self> {
    Box::new(Self::Container(val))
  }

  pub fn type_name(name: String, type_params: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::TypeName { name, type_params })
  }

  pub fn one_of(typerefs: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::OneOf { typerefs })
  }

  pub fn all_of(typerefs: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::AllOf { typerefs })
  }

  pub fn not(typeref: Node<TypeRef>) -> Box<Self> {
    Box::new(Self::Not { typeref })
  }
}
