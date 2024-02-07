use crate::data::LLValue;

#[derive(Clone)]
pub enum LLInterrupt {
  Effect { id: String, args: Vec<LLValue> },
  Exception(LLValue),
}
