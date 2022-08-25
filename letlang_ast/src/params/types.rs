#[derive(NodeAttributes, Debug, Clone, PartialEq)]
#[node_attrs(TypeParamAttributes)]
pub struct TypeParam {
  pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeParamAttributes {
  pub scope_id: usize,
  pub index: usize,
}

impl TypeParam {
  pub fn new(name: String) -> Box<Self> {
    Box::new(Self { name })
  }
}
