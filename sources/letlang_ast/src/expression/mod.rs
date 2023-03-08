mod symbol;
mod literal;
mod containers;
mod calls;
mod operators;
mod pattern;
mod controlflow;
mod tailrec;
mod messaging;

pub use self::{
  symbol::Symbol,
  literal::{Literal, Atom},
  containers::{
    Structure,
    Tuple,
    List,
  },
  calls::{
    EffectCall,
    FunctionCall,
    SpawnCall,
  },
  operators::{
    GenericResolve,
    MemberAccess,
    TypeCheck,
    UnaryOperation,
    BinaryOperation,
    PatternMatch,
  },
  pattern::{
    Pattern,
    PatternAttributes,
    TuplePattern,
    StructPattern,
    ListPattern,
    ListHeadTailPattern,
  },
  controlflow::{
    FlowMatch,
    FlowConditional,
    FlowDoBlock,
    EffectPattern,
    EffectPatternAttributes,
  },
  tailrec::{
    TailRecFinal,
    TailRecRecurse,
  },
  messaging::Receive,
};

use crate::Node;

#[derive(NodeAttributes, Debug, Clone, PartialEq)]
#[node_attrs(ExpressionAttributes)]
pub enum Expression {
  Symbol(Symbol),
  Literal(Node<Literal>),
  Structure(Structure),
  Tuple(Tuple),
  List(List),
  EffectCall(EffectCall),
  FunctionCall(FunctionCall),
  SpawnCall(SpawnCall),
  GenericResolve(GenericResolve),
  MemberAccess(MemberAccess),
  TypeCheck(TypeCheck),
  UnaryOperation(UnaryOperation),
  BinaryOperation(BinaryOperation),
  PatternMatch(PatternMatch),
  TailRecFinal(TailRecFinal),
  TailRecRecurse(TailRecRecurse),
  FlowMatch(FlowMatch),
  FlowConditional(FlowConditional),
  FlowDoBlock(FlowDoBlock),
  Receive(Receive),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionAttributes {
  pub scope_id: usize,
}
