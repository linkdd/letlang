mod literal;
mod pattern;

pub use self::{
  literal::Literal,
  pattern::Pattern,
};

use crate::ast::{
  Node,
  types::TypeRef,
  statement::Proposition,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
  Literal(Node<Literal>),
  Structure {
    members: Vec<(String, Node<Expression>)>,
  },
  Tuple {
    members: Vec<Node<Expression>>,
  },
  List {
    items: Vec<Node<Expression>>,
  },
  Symbol(Vec<String>),
  EffectCall {
    effect_name: Vec<String>,
    params: Vec<Node<Expression>>,
  },
  FunctionCall {
    func: Node<Expression>,
    params: Vec<Node<Expression>>,
  },
  SpawnCall {
    func: Node<Expression>,
    params: Vec<Node<Expression>>,
  },
  GenericResolve {
    symbol: Node<Expression>,
    type_params: Vec<Node<TypeRef>>,
  },
  MemberAccess {
    lhs: Node<Expression>,
    rhs: String,
  },
  TypeCheck {
    lhs: Node<Expression>,
    rhs: Node<TypeRef>
  },
  UnaryOperation {
    op: &'static str,
    expr: Node<Expression>,
  },
  BinaryOperation {
    lhs: Node<Expression>,
    op: &'static str,
    rhs: Node<Expression>,
  },
  PatternMatch {
    lhs: Node<Pattern>,
    rhs: Node<Expression>,
  },
  Loop {
    label: String,
    body: Vec<Node<Proposition>>,
  },
  Break {
    label: String,
    value: Node<Expression>,
  },
  FlowMatch {
    expr: Node<Expression>,
    cases: Vec<(Node<Pattern>, Vec<Node<Proposition>>)>,
  },
  FlowConditional {
    cases: Vec<(Node<Expression>, Vec<Node<Proposition>>)>,
    else_case: Vec<Node<Proposition>>,
  },
  Receive {
    cases: Vec<(Node<Pattern>, Vec<Node<Proposition>>)>,
    after: Option<(Node<Expression>, Vec<Node<Proposition>>)>,
  },
}

impl Expression {
  pub fn literal(node: Node<Literal>) -> Box<Self> {
    Box::new(Self::Literal(node))
  }

  pub fn structure(members: Vec<(String, Node<Expression>)>) -> Box<Self> {
    Box::new(Self::Structure { members })
  }

  pub fn tuple(members: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::Tuple { members })
  }

  pub fn list(items: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::List { items })
  }

  pub fn symbol(path: Vec<String>) -> Box<Self> {
    Box::new(Self::Symbol(path))
  }

  pub fn effect_call(effect_name: Vec<String>, params: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::EffectCall { effect_name, params })
  }

  pub fn function_call(func: Node<Expression>, params: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::FunctionCall { func, params })
  }

  pub fn spawn_call(func: Node<Expression>, params: Vec<Node<Expression>>) -> Box<Self> {
    Box::new(Self::SpawnCall { func, params })
  }

  pub fn generic_resolve(symbol: Node<Expression>, type_params: Vec<Node<TypeRef>>) -> Box<Self> {
    Box::new(Self::GenericResolve { symbol, type_params })
  }

  pub fn member_access(lhs: Node<Expression>, rhs: String) -> Box<Self> {
    Box::new(Self::MemberAccess { lhs, rhs })
  }

  pub fn type_check(lhs: Node<Expression>, rhs: Node<TypeRef>) -> Box<Self> {
    Box::new(Self::TypeCheck { lhs, rhs })
  }

  pub fn unary_op(op: &'static str, expr: Node<Expression>) -> Box<Self> {
    Box::new(Self::UnaryOperation { op, expr })
  }

  pub fn binary_op(op: &'static str, lhs: Node<Expression>, rhs: Node<Expression>) -> Box<Self> {
    Box::new(Self::BinaryOperation { lhs, op, rhs })
  }

  pub fn pattern_match(lhs: Node<Pattern>, rhs: Node<Expression>) -> Box<Self> {
    Box::new(Self::PatternMatch { lhs, rhs })
  }

  pub fn loop_block(label: String, body: Vec<Node<Proposition>>) -> Box<Self> {
    Box::new(Self::Loop { label, body })
  }

  pub fn loop_break(label: String, value: Node<Expression>) -> Box<Self> {
    Box::new(Self::Break { label, value })
  }

  pub fn flow_match(
    expr: Node<Expression>,
    cases: Vec<(Node<Pattern>, Vec<Node<Proposition>>)>,
  ) -> Box<Self> {
    Box::new(Self::FlowMatch { expr, cases })
  }

  pub fn flow_conditional(
    cases: Vec<(Node<Expression>, Vec<Node<Proposition>>)>,
    else_case: Vec<Node<Proposition>>,
  ) -> Box<Self> {
    Box::new(Self::FlowConditional { cases, else_case })
  }

  pub fn receive(
    cases: Vec<(Node<Pattern>, Vec<Node<Proposition>>)>,
    after: Option<(Node<Expression>, Vec<Node<Proposition>>)>,
  ) -> Box<Self> {
    Box::new(Self::Receive { cases, after })
  }
}
