use crate::ast::{
  Node,
  expression::{Symbol, Literal},
  types::TypeRef,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeVal {
  pub val: Node<Literal>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeName {
  pub symbol: Symbol,
  pub type_params: Vec<Node<TypeRef>>
}

impl TypeRef {
  pub fn value(val: Node<Literal>) -> Box<Self> {
    Box::new(Self::Value(TypeVal { val }))
  }

  pub fn type_name(path: Vec<String>, type_params: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::TypeName(TypeName { symbol: Symbol(path), type_params }))
  }
}
