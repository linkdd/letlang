use crate::repr::{Atom, Pid};
use crate::core::context::TaskContext;

use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Boolean(bool),
  Number(f64),
  String(String),
  Atom(Atom),

  Tuple(Box<[Value]>),
  List(Vec<Value>),
  Struct(HashMap<String, Value>),

  Pid(Pid),
}

impl Value {
  pub fn to_string(&self, context: &TaskContext) -> String {
    match self {
      Value::Boolean(val) => format!("{}", val),
      Value::Number(val) => format!("{}", val),
      Value::String(val) => format!("{:?}", val),
      Value::Atom(val) => {
        let repr = context.atom_table.lock().unwrap().lookup(val);
        format!("{}", repr)
      },
      Value::Tuple(vals) => {
        let members = vals
          .iter()
          .map(|member| member.to_string(context))
          .collect::<Vec<String>>()
          .join(", ");

        format!("({})", members)
      },
      Value::List(vals) => {
        let items = vals
          .iter()
          .map(|item| item.to_string(context))
          .collect::<Vec<String>>()
          .join(", ");

        format!("[{}]", items)
      },
      Value::Struct(vals) => {
        let entries = vals
          .iter()
          .map(|(key, value)| {
            format!("{}: {}", key, value.to_string(context))
          })
          .collect::<Vec<String>>()
          .join(", ");

        format!("{{{}}}", entries)
      },
      Value::Pid(val) => format!("{:?}", val),
    }
  }
}
