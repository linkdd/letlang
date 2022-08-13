use crate::ast::{
  Node,
  statement::Proposition,
  expression::{Expression, Pattern},
};


#[derive(Debug, Clone, PartialEq)]
pub struct Loop {
  pub label: String,
  pub body: Vec<Node<Proposition>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Break {
  pub label: String,
  pub value: Node<Expression>,
}

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
  pub fn loop_block(label: String, body: Vec<Node<Proposition>>) -> Box<Self> {
    Box::new(Self::Loop(Loop { label, body }))
  }

  pub fn loop_break(label: String, value: Node<Expression>) -> Box<Self> {
    Box::new(Self::Break(Break { label, value }))
  }

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
