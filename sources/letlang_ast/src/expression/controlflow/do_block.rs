use crate::{
  Node,
  statement::Proposition,
  expression::{Expression, Pattern, Symbol},
};

#[derive(Debug, Clone, PartialEq)]
pub struct FlowDoBlock {
  pub body: Vec<Node<Proposition>>,
  pub effect_handlers: Vec<(Node<EffectPattern>, Vec<Node<Proposition>>)>,
  pub exception_handlers: Vec<(Node<Pattern>, Vec<Node<Proposition>>)>,
}

#[derive(NodeAttributes, Debug, Clone, PartialEq)]
#[node_attrs(EffectPatternAttributes)]
pub struct EffectPattern {
  pub effect_name: Symbol,
  pub effect_params: Vec<Node<Pattern>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EffectPatternAttributes {
  pub scope_id: usize,
}

impl EffectPattern {
  pub fn new(
    effect_name: Symbol,
    effect_params: Vec<Node<Pattern>>,
  ) -> Box<Self> {
    Box::new(Self { effect_name, effect_params })
  }
}

impl Expression {
  pub fn flow_do_block(
    body: Vec<Node<Proposition>>,
    effect_handlers: Vec<(Node<EffectPattern>, Vec<Node<Proposition>>)>,
    exception_handlers: Vec<(Node<Pattern>, Vec<Node<Proposition>>)>,
  ) -> Box<Self> {
    Box::new(Self::FlowDoBlock(
      FlowDoBlock { body, effect_handlers, exception_handlers }
    ))
  }
}
