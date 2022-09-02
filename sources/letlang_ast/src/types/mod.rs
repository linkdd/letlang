mod base;
mod containers;
mod function;
mod sum_types;

pub use self::{
  base::{TypeVal, TypeName},
  containers::{StructDefinition, TupleDefinition},
  function::FunctionSignature,
  sum_types::{OneOfType, AllOfType, NotType},
};

#[derive(NodeAttributes, Debug, Clone, PartialEq)]
#[node_attrs(TypeRefAttributes)]
pub enum TypeRef {
  Value(TypeVal),
  TypeName(TypeName),
  StructDefinition(StructDefinition),
  TupleDefinition(TupleDefinition),
  FunctionSignature(FunctionSignature),
  OneOf(OneOfType),
  AllOf(AllOfType),
  Not(NotType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeRefAttributes {
  pub scope_id: usize,
}