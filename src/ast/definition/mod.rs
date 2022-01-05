mod constant;
mod import;
mod export;
mod function;
mod class;
mod effect;

pub use self::{
  constant::*,
  import::*,
  export::*,
  function::*,
  class::*,
  effect::*,
};

#[derive(Debug, Clone)]
pub enum Definition {
  Constant(ConstantDefinition),
  Import(ImportDefinition),
  Export(ExportDefinition),
  Function(FunctionDefinition),
  Class(ClassDefinition),
  Effect(EffectDefinition),
}

pub type DefinitionNode = Box<Definition>;
