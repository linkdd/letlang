#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
  Atom(String),
  Boolean(bool),
  Number(f64),
  String(String),
}
