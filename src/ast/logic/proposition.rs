use crate::ast::{VariableNameNode, TypeConstraintNode, ExpressionNode};

#[derive(Debug, Clone)]
pub enum Proposition {
  ThereIs { var_name: VariableNameNode, var_type: TypeConstraintNode },
  ForAll { var_name: VariableNameNode, var_type: TypeConstraintNode },
  Expression(ExpressionNode),
}

pub type PropositionNode = Box<Proposition>;
