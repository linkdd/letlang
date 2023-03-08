use crate::builtins::prelude::*;
use crate::repr::Value;
use crate::core::{
  function::{FunctionCoroutine, FunctionInterruption},
  context::TaskContext,
};

use std::sync::Arc;
use tokio::sync::Mutex;
use std::cmp::Ordering;


pub async fn eq(
  _context: Arc<Mutex<TaskContext>>,
  _co: &FunctionCoroutine,
  a: &Value,
  b: &Value,
) -> Value {
  Value::Boolean(a == b)
}

pub async fn ne(
  _context: Arc<Mutex<TaskContext>>,
  _co: &FunctionCoroutine,
  a: &Value,
  b: &Value,
) -> Value {
  Value::Boolean(a != b)
}

pub async fn lt(
  context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  a: &Value,
  b: &Value,
) -> Value {
  match (a, b) {
    (Value::Boolean(val_a), Value::Boolean(val_b)) => {
      Value::Boolean(*val_a < *val_b)
    },
    (Value::Number(val_a), Value::Number(val_b)) => {
      Value::Boolean(*val_a < *val_b)
    },
    (Value::String(val_a), Value::String(val_b)) => {
      match val_a.cmp(val_b) {
        Ordering::Less => Value::Boolean(true),
        _ => Value::Boolean(false),
      }
    },
    (Value::Atom(val_a), Value::Atom(val_b)) => {
      let (val_a_repr, val_b_repr) = async {
        let ctx = context.lock().await;
        let atom_table = ctx.atom_table.lock().unwrap();
        let a_repr = atom_table.lookup(val_a).to_string();
        let b_repr = atom_table.lookup(val_b).to_string();
        (a_repr, b_repr)
      }.await;

      match val_a_repr.cmp(&val_b_repr) {
        Ordering::Less => Value::Boolean(true),
        _ => Value::Boolean(false),
      }
    },
    (Value::Tuple(_val_a), Value::Tuple(_val_b)) => {
      todo!();
    },
    (Value::List(_val_a), Value::List(_val_b)) => {
      todo!();
    },
    _ => {
      let err = OperationError::OperationNotSupported;
      let exc = err.to_letlang_value(context.clone()).await;
      co.yield_(FunctionInterruption::Exception(exc)).await;
      unreachable!();
    }
  }
}


pub async fn lte(
  context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  a: &Value,
  b: &Value,
) -> Value {
  match (a, b) {
    (Value::Boolean(val_a), Value::Boolean(val_b)) => {
      Value::Boolean(*val_a <= *val_b)
    },
    (Value::Number(val_a), Value::Number(val_b)) => {
      Value::Boolean(*val_a <= *val_b)
    },
    (Value::String(val_a), Value::String(val_b)) => {
      match val_a.cmp(val_b) {
        Ordering::Less => Value::Boolean(true),
        Ordering::Equal => Value::Boolean(true),
        _ => Value::Boolean(false),
      }
    },
    (Value::Atom(val_a), Value::Atom(val_b)) => {
      let (val_a_repr, val_b_repr) = async {
        let ctx = context.lock().await;
        let atom_table = ctx.atom_table.lock().unwrap();
        let a_repr = atom_table.lookup(val_a).to_string();
        let b_repr = atom_table.lookup(val_b).to_string();
        (a_repr, b_repr)
      }.await;

      match val_a_repr.cmp(&val_b_repr) {
        Ordering::Less => Value::Boolean(true),
        Ordering::Equal => Value::Boolean(true),
        _ => Value::Boolean(false),
      }
    },
    (Value::Tuple(_val_a), Value::Tuple(_val_b)) => {
      todo!();
    },
    (Value::List(_val_a), Value::List(_val_b)) => {
      todo!();
    },
    _ => {
      let err = OperationError::OperationNotSupported;
      let exc = err.to_letlang_value(context.clone()).await;
      co.yield_(FunctionInterruption::Exception(exc)).await;
      unreachable!();
    }
  }
}

pub async fn gt(
  context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  a: &Value,
  b: &Value,
) -> Value {
  match (a, b) {
    (Value::Boolean(val_a), Value::Boolean(val_b)) => {
      Value::Boolean(*val_a > *val_b)
    },
    (Value::Number(val_a), Value::Number(val_b)) => {
      Value::Boolean(*val_a > *val_b)
    },
    (Value::String(val_a), Value::String(val_b)) => {
      match val_a.cmp(val_b) {
        Ordering::Greater => Value::Boolean(true),
        _ => Value::Boolean(false),
      }
    },
    (Value::Atom(val_a), Value::Atom(val_b)) => {
      let (val_a_repr, val_b_repr) = async {
        let ctx = context.lock().await;
        let atom_table = ctx.atom_table.lock().unwrap();
        let a_repr = atom_table.lookup(val_a).to_string();
        let b_repr = atom_table.lookup(val_b).to_string();
        (a_repr, b_repr)
      }.await;

      match val_a_repr.cmp(&val_b_repr) {
        Ordering::Greater => Value::Boolean(true),
        _ => Value::Boolean(false),
      }
    },
    (Value::Tuple(_val_a), Value::Tuple(_val_b)) => {
      todo!();
    },
    (Value::List(_val_a), Value::List(_val_b)) => {
      todo!();
    },
    _ => {
      let err = OperationError::OperationNotSupported;
      let exc = err.to_letlang_value(context.clone()).await;
      co.yield_(FunctionInterruption::Exception(exc)).await;
      unreachable!();
    }
  }
}


pub async fn gte(
  context: Arc<Mutex<TaskContext>>,
  co: &FunctionCoroutine,
  a: &Value,
  b: &Value,
) -> Value {
  match (a, b) {
    (Value::Boolean(val_a), Value::Boolean(val_b)) => {
      Value::Boolean(*val_a >= *val_b)
    },
    (Value::Number(val_a), Value::Number(val_b)) => {
      Value::Boolean(*val_a >= *val_b)
    },
    (Value::String(val_a), Value::String(val_b)) => {
      match val_a.cmp(val_b) {
        Ordering::Greater => Value::Boolean(true),
        Ordering::Equal => Value::Boolean(true),
        _ => Value::Boolean(false),
      }
    },
    (Value::Atom(val_a), Value::Atom(val_b)) => {
      let (val_a_repr, val_b_repr) = async {
        let ctx = context.lock().await;
        let atom_table = ctx.atom_table.lock().unwrap();
        let a_repr = atom_table.lookup(val_a).to_string();
        let b_repr = atom_table.lookup(val_b).to_string();
        (a_repr, b_repr)
      }.await;

      match val_a_repr.cmp(&val_b_repr) {
        Ordering::Greater => Value::Boolean(true),
        Ordering::Equal => Value::Boolean(true),
        _ => Value::Boolean(false),
      }
    },
    (Value::Tuple(_val_a), Value::Tuple(_val_b)) => {
      todo!();
    },
    (Value::List(_val_a), Value::List(_val_b)) => {
      todo!();
    },
    _ => {
      let err = OperationError::OperationNotSupported;
      let exc = err.to_letlang_value(context.clone()).await;
      co.yield_(FunctionInterruption::Exception(exc)).await;
      unreachable!();
    }
  }
}
