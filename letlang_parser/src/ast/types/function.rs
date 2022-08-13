use crate::ast::{
  Node,
  types::TypeRef,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
  pub params: Vec<Node<TypeRef>>,
  pub return_type: Node<TypeRef>
}

impl TypeRef {
  pub fn function_signature(params: Vec<Node<TypeRef>>, return_type: Node<TypeRef>) -> Box<Self> {
    Box::new(Self::FunctionSignature(FunctionSignature { params, return_type }))
  }
}
