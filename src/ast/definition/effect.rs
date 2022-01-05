use crate::ast::{TypeConstraintNode, TypeParamNode};

#[derive(Debug, Clone)]
pub struct EffectDefinition {
  pub name: String,
  pub params: Vec<TypeParamNode>,
  pub return_type: TypeConstraintNode,
}
