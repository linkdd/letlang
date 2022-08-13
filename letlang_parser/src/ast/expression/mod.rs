mod symbol;
mod literal;
mod containers;
mod calls;
mod operators;
mod pattern;
mod controlflow;
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
    TuplePattern,
    StructPattern,
    ListPattern,
    ListHeadTailPattern,
  },
  controlflow::{
    Loop,
    Break,
    FlowMatch,
    FlowConditional,
  },
  messaging::Receive,
};

use crate::ast::Node;

#[derive(Debug, Clone, PartialEq)]
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
  Loop(Loop),
  Break(Break),
  FlowMatch(FlowMatch),
  FlowConditional(FlowConditional),
  Receive(Receive),
}
