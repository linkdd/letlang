use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pid(Uuid);

impl Pid {
  pub fn new() -> Self {
    Self(Uuid::new_v4())
  }
}
