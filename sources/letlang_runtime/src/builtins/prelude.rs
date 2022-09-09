use crate::repr::Value;
use crate::core::context::TaskContext;

use std::sync::Arc;
use tokio::sync::Mutex;


#[derive(Debug)]
pub enum OperationError {
  TypeError { expected: String, got: String },
  MatchError { value: String },
  CallParamCountError { expected: usize, got: usize },
  DivisionByZero,
}

unsafe impl Sync for OperationError {}
unsafe impl Send for OperationError {}


impl OperationError {
  pub async fn to_letlang_value(&self, context: Arc<Mutex<TaskContext>>) -> Value {
    match self {
      Self::TypeError { expected, got } => {
        let type_error_atom = async {
          let context_ref = context.lock().await;
          let atom_table = context_ref.atom_table.lock().unwrap();
          atom_table.from_repr("@type_error")
        }.await;

        Value::Tuple(Box::new([
          Value::Atom(type_error_atom),
          Value::String(format!(
            "expected type {}, got value {}",
            expected,
            got,
          )),
        ]))
      },
      Self::MatchError { value } => {
        let match_error_atom = async {
          let context_ref = context.lock().await;
          let atom_table = context_ref.atom_table.lock().unwrap();
          atom_table.from_repr("@match_error")
        }.await;

        Value::Tuple(Box::new([
          Value::Atom(match_error_atom),
          Value::String(format!(
            "no match on lefthand side for {}",
            value,
          )),
        ]))
      },
      Self::CallParamCountError { expected, got } => {
        let type_error_atom = async {
          let context_ref = context.lock().await;
          let atom_table = context_ref.atom_table.lock().unwrap();
          atom_table.from_repr("@type_error")
        }.await;

        Value::Tuple(Box::new([
          Value::Atom(type_error_atom),
          Value::String(format!(
            "expected {} arguments, got {}",
            expected,
            got,
          )),
        ]))
      },
      Self::DivisionByZero => {
        let error_atom = async {
          let context_ref = context.lock().await;
          let atom_table = context_ref.atom_table.lock().unwrap();
          atom_table.from_repr("@error")
        }.await;

        let division_by_zero_atom = async {
          let context_ref = context.lock().await;
          let atom_table = context_ref.atom_table.lock().unwrap();
          atom_table.from_repr("@division_by_zero")
        }.await;

        Value::Tuple(Box::new([
          Value::Atom(error_atom),
          Value::Atom(division_by_zero_atom),
        ]))
      }
    }
  }
}
