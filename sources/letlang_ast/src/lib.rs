#[macro_use] extern crate letlang_derive;

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

pub trait NodeAttributes {
  type Attributes;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node<T: NodeAttributes + 'static> {
  pub location: LocationInfo,
  pub data: Box<T>,
  pub attrs: Option<T::Attributes>,
}

impl<T: NodeAttributes + 'static> Node<T> {
  pub fn new(location: LocationInfo, data: Box<T>) -> Self {
    Self { location, data, attrs: None }
  }
}
