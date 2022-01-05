use crate::ast::{ExpressionNode, VariableNameNode};

#[derive(Debug, Clone)]
pub enum AccessOperation {
  GetItem {
    source: ExpressionNode,
    index: ExpressionNode,
  },
  GetField {
    source: ExpressionNode,
    field: VariableNameNode,
  },
}
