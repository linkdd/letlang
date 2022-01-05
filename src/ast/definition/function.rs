use crate::ast::{TypeConstraintNode, TypeParamNode, StatementNode};

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
  pub func_name: String,
  pub type_params: Vec<TypeParamNode>,
  pub func_params: Vec<TypeParamNode>,
  pub return_type: TypeConstraintNode,
  pub body: Vec<StatementNode>,
}
