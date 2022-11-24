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
  OperationNotSupported,
  ConstraintError(String),
  Undefined(String),
  TailRecursion(Value),
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

        let unexpected_atom = async {
          let context_ref = context.lock().await;
          let atom_table = context_ref.atom_table.lock().unwrap();
          atom_table.from_repr("@unexpected")
        }.await;

        Value::Tuple(Box::new([
          Value::Atom(type_error_atom),
          Value::Tuple(Box::new([
            Value::Atom(unexpected_atom),
            Value::String(expected.clone()),
            Value::String(got.clone()),
          ])),
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
          Value::String(value.clone()),
        ]))
      },
      Self::CallParamCountError { expected, got } => {
        let type_error_atom = async {
          let context_ref = context.lock().await;
          let atom_table = context_ref.atom_table.lock().unwrap();
          atom_table.from_repr("@type_error")
        }.await;

        let arity_atom = async {
          let context_ref = context.lock().await;
          let atom_table = context_ref.atom_table.lock().unwrap();
          atom_table.from_repr("@arity")
        }.await;

        Value::Tuple(Box::new([
          Value::Atom(type_error_atom),
          Value::Tuple(Box::new([
            Value::Atom(arity_atom),
            Value::String(format!("{expected}")),
            Value::String(format!("{got}")),
          ])),
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
      },
      Self::ConstraintError(symbol) => {
        let type_error_atom = async {
          let context_ref = context.lock().await;
          let atom_table = context_ref.atom_table.lock().unwrap();
          atom_table.from_repr("@type_error")
        }.await;

        let constraint_atom = async {
          let context_ref = context.lock().await;
          let atom_table = context_ref.atom_table.lock().unwrap();
          atom_table.from_repr("@constraint")
        }.await;

        Value::Tuple(Box::new([
          Value::Atom(type_error_atom),
          Value::Tuple(Box::new([
            Value::Atom(constraint_atom),
            Value::String(symbol.clone()),
          ])),
        ]))
      },
      Self::OperationNotSupported => {
        let type_error_atom = async {
          let context_ref = context.lock().await;
          let atom_table = context_ref.atom_table.lock().unwrap();
          atom_table.from_repr("@type_error")
        }.await;

        let operation_not_supported_atom = async {
          let context_ref = context.lock().await;
          let atom_table = context_ref.atom_table.lock().unwrap();
          atom_table.from_repr("@operation_not_supported")
        }.await;

        Value::Tuple(Box::new([
          Value::Atom(type_error_atom),
          Value::Atom(operation_not_supported_atom),
        ]))
      },
      Self::Undefined(symbol) => {
        let error_atom = async {
          let context_ref = context.lock().await;
          let atom_table = context_ref.atom_table.lock().unwrap();
          atom_table.from_repr("@error")
        }.await;

        let undefined_atom = async {
          let context_ref = context.lock().await;
          let atom_table = context_ref.atom_table.lock().unwrap();
          atom_table.from_repr("@undefined")
        }.await;

        Value::Tuple(Box::new([
          Value::Atom(error_atom),
          Value::Tuple(Box::new([
            Value::Atom(undefined_atom),
            Value::String(symbol.clone()),
          ]))
        ]))
      },
      Self::TailRecursion(value) => {
        let type_error_atom = async {
          let context_ref = context.lock().await;
          let atom_table = context_ref.atom_table.lock().unwrap();
          atom_table.from_repr("@type_error")
        }.await;

        let tailrec_atom = async {
          let context_ref = context.lock().await;
          let atom_table = context_ref.atom_table.lock().unwrap();
          atom_table.from_repr("@tailrec")
        }.await;

        Value::Tuple(Box::new([
          Value::Atom(type_error_atom),
          Value::Tuple(Box::new([
            Value::Atom(tailrec_atom),
            value.clone(),
          ])),
        ]))
      }
    }
  }
}
