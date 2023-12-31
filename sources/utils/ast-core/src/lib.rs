#[derive(Debug, Clone, PartialEq)]
pub struct Node<T, M> {
  meta: M,
  data: Box<T>,
}

impl<T, M> Node<T, M> {
  pub fn new(meta: M, data: T) -> Self {
    Self {
      meta,
      data: Box::new(data),
    }
  }

  pub fn get_meta(&self) -> &M {
    &self.meta
  }

  pub fn get_data(&self) -> &T {
    self.data.as_ref()
  }
}

pub trait Model<T, M> {
  type Result;
  type Extra;

  fn visit(&mut self, node: &Node<T, M>, extra: Self::Extra) -> Self::Result;
}
