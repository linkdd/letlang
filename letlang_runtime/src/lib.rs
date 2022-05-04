#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveValue {
  Boolean(bool),
  Number(f64),
  String(String),
  Atom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Primitive(PrimitiveValue),
  Tuple(Vec<Value>),
}

pub trait Type {
  fn has(&self, llval: &Value) -> bool;
}

pub trait Function {
  fn call(&self) -> Value;
}

pub trait Effect {
  fn perform(&self) -> Value;
}

pub mod types;
pub mod operations;
