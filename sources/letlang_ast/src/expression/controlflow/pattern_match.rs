use crate::{
  Node,
  statement::Proposition,
  expression::{Expression, Pattern},
};

#[derive(Debug, Clone, PartialEq)]
pub struct FlowMatch {
  pub expr: Node<Expression>,
  pub cases: Vec<(Node<Pattern>, Vec<Node<Proposition>>)>,
}

impl Expression {
  pub fn flow_match(
    expr: Node<Expression>,
    cases: Vec<(Node<Pattern>, Vec<Node<Proposition>>)>,
  ) -> Box<Self> {
    Box::new(Self::FlowMatch(FlowMatch { expr, cases }))
  }
}
