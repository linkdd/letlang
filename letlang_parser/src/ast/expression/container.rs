use std::collections::HashMap;
use serde::Serialize;

use crate::ast::{
  Node,
  expression::Expression,
  types::TypeRef,
};

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type")]
pub enum Container {
  TypeRef(Node<TypeRef>),
  Tuple(Vec<Node<Expression>>),
  List(Vec<Node<Expression>>),
  Set(Vec<Node<Expression>>),
  SetBuilder { identifier: String, source_type: Node<TypeRef>, predicate: Node<Expression> },
  Map(HashMap<String, Node<Expression>>),
}

impl Container {
  pub fn type_ref(type_ref: Node<TypeRef>) -> Box<Self> {
    Box::new(Self::TypeRef(type_ref))
  }

  pub fn tuple(items: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::Tuple(items))
  }

  pub fn list(items: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::List(items))
  }

  pub fn set(items: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::Set(items))
  }

  pub fn set_builder(
    identifier: String,
    source_type: Node<TypeRef>,
    predicate: Node<Expression>,
  ) -> Box<Self> {
    Box::new(Self::SetBuilder { identifier, source_type, predicate })
  }

  pub fn map(items: HashMap<String, Node<Expression>>) -> Box<Self> {
    Box::new(Self::Map(items))
  }
}
