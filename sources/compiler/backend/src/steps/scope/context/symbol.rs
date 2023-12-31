use super::EnvRef;

#[derive(Clone)]
pub enum SymbolKind {
  Module {
    env: EnvRef,
  },
  Class {
    type_arity: usize,
    local_index: Option<usize>,
  },
  Effect {
    type_arity: usize,
    call_arity: usize,
  },
  Function {
    type_arity: usize,
    call_arity: usize,
  },
  LocalVariable,
}
