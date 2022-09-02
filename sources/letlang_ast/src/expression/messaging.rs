use crate::{
  Node,
  statement::Proposition,
  expression::{Expression, Pattern},
};


#[derive(Debug, Clone, PartialEq)]
pub struct Receive {
  pub cases: Vec<(Node<Pattern>, Vec<Node<Proposition>>)>,
  pub after: Option<(Node<Expression>, Vec<Node<Proposition>>)>,
}


impl Expression {
  pub fn receive(
    cases: Vec<(Node<Pattern>, Vec<Node<Proposition>>)>,
    after: Option<(Node<Expression>, Vec<Node<Proposition>>)>,
  ) -> Box<Self> {
    Box::new(Self::Receive(Receive { cases, after }))
  }
}
