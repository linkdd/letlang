use crate::repr::Value;

#[derive(Clone)]
pub enum FunctionInterruption {
  Effect { name: String, args: Vec<Value> },
  Exception(Value),
}
