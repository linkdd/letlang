use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "_type", content = "value")]
pub enum Literal {
  Boolean(bool),
  Number(f64),
  Atom(String),
  String(String),
}

impl Literal {
  pub fn boolean(val: bool) -> Box<Self> {
    Box::new(Self::Boolean(val))
  }

  pub fn number(val: f64) -> Box<Self> {
    Box::new(Self::Number(val))
  }

  pub fn atom(repr: String) -> Box<Self> {
    Box::new(Self::Atom(repr))
  }

  pub fn string(repr: String) -> Box<Self> {
    Box::new(Self::String(repr))
  }
}
