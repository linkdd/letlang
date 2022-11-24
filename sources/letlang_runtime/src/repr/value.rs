use crate::repr::{Atom, Pid};
use crate::core::context::TaskContext;

use async_recursion::async_recursion;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;


#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Boolean(bool),
  Number(f64),
  String(String),
  Atom(Atom),

  Tuple(Box<[Value]>),
  List(Vec<Value>),
  Struct(HashMap<String, Value>),

  TailRecFinal(Box<Value>),
  TailRecRecurse(Vec<Value>),

  Pid(Pid),
}

impl Value {
  #[async_recursion]
  pub async fn to_string(&self, context: Arc<Mutex<TaskContext>>) -> String {
    match self {
      Value::Boolean(val) => format!("{}", val),
      Value::Number(val) => format!("{}", val),
      Value::String(val) => format!("{:?}", val),
      Value::Atom(val) => {
        let context = context.lock().await;
        let repr = context.atom_table.lock().unwrap().lookup(val);
        format!("{}", repr)
      },
      Value::Tuple(vals) => {
        let mut members = vec![];

        for member in vals.iter() {
          members.push(member.to_string(context.clone()).await);
        }

        format!("({})", members.join(", "))
      },
      Value::List(vals) => {
        let mut items = vec![];

        for item in vals.iter() {
          items.push(item.to_string(context.clone()).await);
        }

        format!("[{}]", items.join(", "))
      },
      Value::Struct(vals) => {
        let mut entries = vec![];

        for (key, value) in vals.iter() {
          entries.push(format!(
            "{}: {}",
            key,
            value.to_string(context.clone()).await,
          ));
        }

        format!("{{{}}}", entries.join(", "))
      },
      Value::Pid(val) => {
        format!("{:?}", val)
      },
      Value::TailRecFinal(value) => {
        format!("final[{}]", value.to_string(context.clone()).await)
      },
      Value::TailRecRecurse(args) => {
        let mut items = vec![];

        for item in args.iter() {
          items.push(item.to_string(context.clone()).await);
        }

        format!("recurse[{}]", items.join(", "))
      }
    }
  }
}
