#[derive(Clone)]
pub struct LLPid(u128);

impl std::fmt::Display for LLPid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Pid({})", self.0)
  }
}
