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

#[derive(Debug, Clone, PartialEq)]
pub struct FlowConditional {
  pub cases: Vec<(Node<Expression>, Vec<Node<Proposition>>)>,
  pub else_case: Vec<Node<Proposition>>,
}


impl Expression {
  pub fn flow_match(
    expr: Node<Expression>,
    cases: Vec<(Node<Pattern>, Vec<Node<Proposition>>)>,
  ) -> Box<Self> {
    Box::new(Self::FlowMatch(FlowMatch { expr, cases }))
  }

  pub fn flow_conditional(
    cases: Vec<(Node<Expression>, Vec<Node<Proposition>>)>,
    else_case: Vec<Node<Proposition>>,
  ) -> Box<Self> {
    Box::new(Self::FlowConditional(FlowConditional { cases, else_case }))
  }
}
