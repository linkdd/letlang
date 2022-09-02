use crate::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol(pub Vec<String>);


impl Expression {
  pub fn symbol(path: Vec<String>) -> Box<Self> {
    Box::new(Self::Symbol(Symbol(path)))
  }
}

impl Symbol {
  pub fn scope(&self) -> Option<String> {
    let Symbol(path) = self;

    if path.len() > 1 {
      Some(path[0..path.len()-1].join("::"))
    }
    else {
      None
    }
  }

  pub fn name(&self) -> String {
    let Symbol(path) = self;
    path.last().unwrap().clone()
  }

  pub fn to_string(&self) -> String {
    let Symbol(path) = self;
    path.join("::")
  }
}
