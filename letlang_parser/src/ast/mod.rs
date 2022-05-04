pub mod types;
pub mod expression;
pub mod funcs;
pub mod class;
pub mod statement;
pub mod unit;

pub use self::{
  statement::Statement,
  unit::Unit,
};

use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Node<T> {
  pub filename: String,
  pub start: usize,
  pub end: usize,

  pub data: Box<T>,
}

impl<T> Node<T> {
  pub fn new(filename: String, start: usize, end: usize, data: Box<T>) -> Self {
    Self { filename, start, end, data }
  }
}
