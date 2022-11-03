use crate::{Node, expression::Expression};

#[derive(Debug, Clone, PartialEq)]
pub struct TailRecFinal {
  pub value: Node<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TailRecRecurse {
  pub args: Vec<Node<Expression>>,
}


impl Expression {
  pub fn tailrec_final(value: Node<Expression>) -> Box<Self> {
    Box::new(Self::TailRecFinal(TailRecFinal { value }))
  }

  pub fn tailrec_recurse(args: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::TailRecRecurse(TailRecRecurse { args }))
  }
}
