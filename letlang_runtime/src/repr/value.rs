use crate::repr::{Atom, Pid};

use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Boolean(bool),
  Number(f64),
  String(String),
  Atom(Atom),

  Tuple(Box<[Value]>),
  List(Vec<Value>),
  Struct(HashMap<String, Value>),

  Pid(Pid),
}


impl std::fmt::Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Boolean(val) => write!(f, "{}", val),
      Value::Number(val) => write!(f, "{}", val),
      Value::String(val) => write!(f, "{}", val),
      Value::Atom(val) => write!(f, "{:?}", val),
      Value::Tuple(vals) => {
        todo!()
      },
      Value::List(vals) => {
        todo!()
      },
      Value::Struct(vals) => {
        todo!()
      },
      Value::Pid(val) => write!(f, "{:?}", val),
    }
  }
}