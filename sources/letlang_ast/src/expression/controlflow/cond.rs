use crate::{
  Node,
  statement::Proposition,
  expression::Expression,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FlowConditional {
  pub cases: Vec<(Node<Expression>, Vec<Node<Proposition>>)>,
  pub else_case: Vec<Node<Proposition>>,
}

impl Expression {
  pub fn flow_conditional(
    cases: Vec<(Node<Expression>, Vec<Node<Proposition>>)>,
    else_case: Vec<Node<Proposition>>,
  ) -> Box<Self> {
    Box::new(Self::FlowConditional(FlowConditional { cases, else_case }))
  }
}
