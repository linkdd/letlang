use crate::{Node, types::TypeRef};

#[derive(NodeAttributes, Debug, Clone, PartialEq)]
#[node_attrs(CallParamAttributes)]
pub struct CallParam {
  pub param_name: String,
  pub param_type: Node<TypeRef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallParamAttributes {
  pub scope_id: usize,
  pub index: usize,
}

impl CallParam {
  pub fn new(param_name: String, param_type: Node<TypeRef>) -> Box<Self> {
    Box::new(Self { param_name, param_type })
  }
}
