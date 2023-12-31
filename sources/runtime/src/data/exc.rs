use crate::{LLValue, LLAtom};

#[derive(Clone)]
pub enum LLException {
  TypeArity { expected: usize, got: usize },
  FuncArity { expected: usize, got: usize },
  NoClauseMatched,
  TypeError,
}

impl From<LLException> for LLValue {
  fn from(value: LLException) -> Self {
    match value {
      LLException::TypeArity { expected, got } => {
        LLValue::Tuple(Box::new([
          LLValue::Atom(LLAtom::new("@type_arity")),
          LLValue::Tuple(Box::new([
            LLValue::Atom(LLAtom::new("@expected")),
            LLValue::Number(expected as f64),
          ])),
          LLValue::Tuple(Box::new([
            LLValue::Atom(LLAtom::new("@got")),
            LLValue::Number(got as f64),
          ])),
        ]))
      },
      LLException::FuncArity { expected, got } => {
        LLValue::Tuple(Box::new([
          LLValue::Atom(LLAtom::new("@func_arity")),
          LLValue::Tuple(Box::new([
            LLValue::Atom(LLAtom::new("@expected")),
            LLValue::Number(expected as f64),
          ])),
          LLValue::Tuple(Box::new([
            LLValue::Atom(LLAtom::new("@got")),
            LLValue::Number(got as f64),
          ])),
        ]))
      },
      LLException::NoClauseMatched => {
        LLValue::Atom(LLAtom::new("@no_clause_matched"))
      },
      LLException::TypeError => {
        LLValue::Atom(LLAtom::new("@type_error"))
      },
    }
  }
}
