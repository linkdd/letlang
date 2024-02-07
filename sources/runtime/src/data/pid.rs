#[derive(Clone, PartialEq, Eq, Hash)]
pub struct LLPid {
  group_id: u64,
  local_id: u64,
}

impl LLPid {
  pub fn new(group_id: u64, local_id: u64) -> Self {
    Self { group_id, local_id }
  }
}

impl std::fmt::Display for LLPid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Pid({}:{})", self.group_id, self.local_id)
  }
}
