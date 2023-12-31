use std::collections::HashMap;

mod atom;
mod pid;
mod exc;

pub use self::{
  atom::*,
  pid::*,
  exc::*,
};

#[derive(Clone)]
pub enum LLValue {
  Atom(LLAtom),
  Bool(bool),
  Number(f64),
  String(String),
  Tuple(Box<[LLValue]>),
  List(Vec<LLValue>),
  NamedTuple(HashMap<String, LLValue>),
  TailRecRecurse(Vec<LLValue>),
  TailRecFinal(Box<LLValue>),
  Pid(LLPid),
}

impl std::fmt::Display for LLValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Atom(val) => {
        write!(f, "{val}")
      },
      Self::Bool(val) => {
        write!(f, "{val}")
      },
      Self::Number(val) => {
        write!(f, "{val}")
      },
      Self::String(val) => {
        write!(f, "{:?}", val)
      },
      Self::Tuple(items) => {
        let inner = items.iter()
          .map(|val| format!("{val}"))
          .fold(String::new(), |a, b| format!("{a}, {b}"));
        write!(f, "({inner})")
      },
      Self::List(items) => {
        let inner = items.iter()
          .map(|val| format!("{val}"))
          .fold(String::new(), |a, b| format!("{a}, {b}"));

        write!(f, "[{inner}]")
      },
      Self::NamedTuple(items) => {
        let inner = items.iter()
          .map(|(key, val)| format!("{key}: {val}"))
          .fold(String::new(), |a, b| format!("{a}, {b}"));

        write!(f, "{{{inner}}}")
      },
      Self::TailRecRecurse(items) => {
        let inner = items.iter()
          .map(|val| format!("{val}"))
          .fold(String::new(), |a, b| format!("{a}, {b}"));

        write!(f, "recurse[{inner}]")
      },
      Self::TailRecFinal(val) => {
        write!(f, "final[{val}]")
      },
      Self::Pid(val) => {
        write!(f, "{val}")
      }
    }
  }
}
