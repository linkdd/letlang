use crate::ast::{VariableNameNode, ExpressionNode};

#[derive(Debug, Clone)]
pub enum AssignmentOperation {
  Variable {
    lhs: VariableNameNode,
    rhs: ExpressionNode
  },
  TupleDestructuring {
    lhs: Vec<VariableNameNode>,
    rhs: ExpressionNode,
  },
  ListDestructuring {
    head: VariableNameNode,
    tail: VariableNameNode,
    rhs: ExpressionNode,
  },
  ObjectDestructuring {
    lhs: Vec<VariableNameNode>,
    rhs: ExpressionNode,
  }
}
