#[derive(Debug, Clone)]
pub enum Literal {
  Boolean(bool),
  Number(f64),
  String(String),
  Atom(String),
}

pub type LiteralNode = Box<Literal>;
