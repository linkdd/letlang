use nonempty::NonEmpty;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Symbol {
  Class {
    name: String,
    type_arity: usize,
  },
  Function {
    name: String,
    type_arity: usize,
    call_arity: usize,
  },
  Effect {
    name: String,
    type_arity: usize,
    call_arity: usize,
  },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryModuleInterface {
  pub crate_name: String,
  pub module: NonEmpty<String>,
  pub symbols: Vec<Symbol>,
}
