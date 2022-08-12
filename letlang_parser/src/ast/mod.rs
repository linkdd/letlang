pub mod types;
pub mod expression;
pub mod unit;
pub mod statement;
pub mod params;

pub use self::{
  statement::Statement,
  unit::Unit,
};

pub type LocationInfo = (usize, usize);

#[derive(Clone, Debug, PartialEq)]
pub struct Node<T> {
  pub location: LocationInfo,
  pub data: Box<T>,
}

impl<T> Node<T> {
  pub fn new(location: LocationInfo, data: Box<T>) -> Self {
    Self { location, data }
  }
}
