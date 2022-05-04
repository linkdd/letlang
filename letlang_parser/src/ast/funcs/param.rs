use crate::ast::{Node, types::TypeRef};
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct CallParam {
  pub param_name: String,
  pub param_type: Node<TypeRef>,
}

impl CallParam {
  pub fn new(param_name: String, param_type: Node<TypeRef>) -> Box<Self> {
    Box::new(Self { param_name, param_type })
  }
}
