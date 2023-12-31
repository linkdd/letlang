use std::fmt::{Display, Formatter, Result};

pub struct TokenLocation(pub usize, pub usize);

impl Display for TokenLocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "[{};{}]", self.0, self.1)
  }
}
