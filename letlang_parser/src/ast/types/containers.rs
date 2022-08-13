use crate::ast::{
  Node,
  types::TypeRef,
};

#[derive(Debug, Clone, PartialEq)]
pub struct StructDefinition {
  pub members: Vec<(String, Node<TypeRef>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TupleDefinition {
  pub members: Vec<Node<TypeRef>>,
}

impl TypeRef {
  pub fn struct_definition(members: Vec<(String, Node<TypeRef>)>) -> Box<Self> {
    Box::new(Self::StructDefinition(StructDefinition { members }))
  }

  pub fn tuple_definition(members: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::TupleDefinition(TupleDefinition { members }))
  }
}
