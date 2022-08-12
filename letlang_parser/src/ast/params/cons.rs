use crate::ast::{Node, types::TypeRef};

#[derive(Clone, Debug, PartialEq)]
pub struct ConsParam {
  pub param_name: String,
  pub param_type: Node<TypeRef>,
}

impl ConsParam {
  pub fn new(param_name: String, param_type: Node<TypeRef>) -> Box<Self> {
    Box::new(Self { param_name, param_type })
  }
}
