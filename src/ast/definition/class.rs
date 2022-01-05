use crate::ast::{TypeParamNode, PredicateNode};

#[derive(Debug, Clone)]
pub struct ClassDefinition {
  pub class_name: String,
  pub type_params: Vec<TypeParamNode>,
  pub constructor_params: Vec<TypeParamNode>,
  pub check: Vec<PredicateNode>,
}
