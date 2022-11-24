use crate::{Node, types::TypeRef};

#[derive(NodeAttributes, Debug, Clone, PartialEq)]
#[node_attrs(ConsParamAttributes)]
pub struct ConsParam {
  pub param_name: String,
  pub param_type: Node<TypeRef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsParamAttributes {
  pub scope_id: usize,
}

impl ConsParam {
  pub fn new(param_name: String, param_type: Node<TypeRef>) -> Box<Self> {
    Box::new(Self { param_name, param_type })
  }
}
