#[derive(Clone, Debug, PartialEq)]
pub struct TypeParam {
  pub name: String,
}

impl TypeParam {
  pub fn new(name: String) -> Box<Self> {
    Box::new(Self { name })
  }
}
