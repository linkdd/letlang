use nonempty::NonEmpty;
use ast_core::*;

use crate::ast::{Identifier, Type, Pattern, Clause};

mod lit;
mod tuple;
mod list;
mod ops;

pub use self::{
  lit::*,
  tuple::*,
  list::*,
  ops::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<M> {
  Variable(Node<Identifier, M>),
  Literal(Node<Literal, M>),
  Tuple(Vec<Node<TupleItem<M>, M>>),
  NamedTuple(Vec<Node<NamedTupleItem<M>, M>>),
  List(Vec<Node<ListItem<M>, M>>),
  ListHeadTail {
    heads: Vec<Node<ListItem<M>, M>>,
    tail: Node<Expression<M>, M>,
  },
  UnaryOperation(UnaryOperator, Node<Expression<M>, M>),
  BinaryOperation {
    lhs: Node<Expression<M>, M>,
    op: BinaryOperator,
    rhs: Node<Expression<M>, M>,
  },
  MemberAccess {
    lhs: Node<Expression<M>, M>,
    rhs: Node<Identifier, M>,
  },
  TypeCheck {
    lhs: Node<Expression<M>, M>,
    rhs: Node<Type<M>, M>,
    negated: bool,
  },
  PatternBind {
    lhs: Node<Pattern<M>, M>,
    rhs: Node<Expression<M>, M>,
  },
  Let {
    bindings: Vec<(Node<Identifier, M>, Node<Type<M>, M>)>,
    guard: Option<Node<Expression<M>, M>>,
    body: NonEmpty<Node<Expression<M>, M>>,
  },
  Match(Node<Expression<M>, M>, NonEmpty<Node<Clause<M>, M>>),
  Receive {
    clauses: NonEmpty<Node<Clause<M>, M>>,
    after: Option<(Node<Literal, M>, NonEmpty<Node<Expression<M>, M>>)>,
  },
  Cond {
    branches: NonEmpty<(
      Node<Expression<M>, M>,
      NonEmpty<Node<Expression<M>, M>>,
    )>,
    else_branch: NonEmpty<Node<Expression<M>, M>>,
  },
  TailrecRecurse(Vec<Node<Expression<M>, M>>),
  TailrecFinal(Node<Expression<M>, M>),
  FunctionCall {
    name: Node<Identifier, M>,
    type_params: Vec<Node<Type<M>, M>>,
    call_params: Vec<Node<Expression<M>, M>>,
  },
  EffectCall {
    name: Node<Identifier, M>,
    type_params: Vec<Node<Type<M>, M>>,
    call_params: Vec<Node<Expression<M>, M>>,
  },
  ExceptionThrow(Node<Expression<M>, M>),
  ProcessSpawn {
    name: Node<Identifier, M>,
    type_params: Vec<Node<Type<M>, M>>,
    call_params: Vec<Node<Expression<M>, M>>,
  },
  Block(CodeBlock<M>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock<M> {
  pub body: NonEmpty<Node<Expression<M>, M>>,
  pub effect_handlers: Vec<(
    Node<Identifier, M>,
    Vec<Node<Type<M>, M>>,
    NonEmpty<Node<Clause<M>, M>>,
  )>,
  pub exception_handlers: Vec<Node<Clause<M>, M>>,
}
