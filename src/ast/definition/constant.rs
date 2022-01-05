use crate::ast::{VariableNameNode, TypeConstraintNode, ExpressionNode};

#[derive(Debug, Clone)]
pub struct ConstantDefinition {
  pub const_name: VariableNameNode,
  pub const_type: TypeConstraintNode,
  pub const_val: ExpressionNode,
}
