#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveValue {
  Boolean(bool),
  Number(f64),
  String(String),
  Atom(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Primitive(PrimitiveValue),
  Tuple(Vec<Value>),
}
