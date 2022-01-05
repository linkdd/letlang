mod arithmetic;
mod binary;
mod stream;
mod comparison;
mod logic;
mod pipeline;
mod coroutine;
mod effect;
mod assign;
mod controlflow;
mod access;

pub use self::{
  arithmetic::*,
  binary::*,
  stream::*,
  comparison::*,
  logic::*,
  pipeline::*,
  coroutine::*,
  effect::*,
  assign::*,
  controlflow::*,
  access::*,
};


#[derive(Debug, Clone)]
pub enum Operation {
  Arithmetic(ArithmeticOperation),
  Binary(BinaryOperation),
  Stream(StreamOperation),
  Comparison(ComparisonOperation),
  Logic(LogicOperation),
  Pipeline(PipelineOperation),
  Coroutine(CoroutineOperation),
  Effect(EffectOperation),
  Assignment(AssignmentOperation),
  ControlFlow(ControlFlowOperation),
  Access(AccessOperation),
}

pub type OperationNode = Box<Operation>;
