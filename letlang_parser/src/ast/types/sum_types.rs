use crate::ast::{
  Node,
  types::TypeRef,
};

#[derive(Debug, Clone, PartialEq)]
pub struct OneOfType {
  pub typerefs: Vec<Node<TypeRef>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AllOfType {
  pub typerefs: Vec<Node<TypeRef>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NotType {
  pub typeref: Node<TypeRef>,
}

impl TypeRef {
  pub fn one_of(typerefs: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::OneOf(OneOfType { typerefs }))
  }

  pub fn all_of(typerefs: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::AllOf(AllOfType { typerefs }))
  }

  pub fn not(typeref: Node<TypeRef>) -> Box<Self> {
    Box::new(Self::Not(NotType { typeref }))
  }
}
