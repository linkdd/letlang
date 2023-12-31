use siphasher::sip::SipHasher;

#[derive(Clone)]
pub struct LLAtom {
  repr: &'static str,
  hash: u64,
}

impl LLAtom {
  pub fn new(repr: &'static str) -> Self {
    let hasher = SipHasher::new();
    let hash = hasher.hash(repr.as_bytes());

    Self { repr, hash }
  }

  pub fn new_explicit(repr: &'static str, hash: u64) -> Self {
    Self { repr, hash }
  }
}

impl PartialEq for LLAtom {
  fn eq(&self, other: &Self) -> bool {
    self.hash == other.hash
  }
}

impl Eq for LLAtom {}

impl std::fmt::Display for LLAtom {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.repr)
  }
}
