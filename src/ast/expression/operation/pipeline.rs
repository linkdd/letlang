use crate::ast::{ExpressionNode, FunctionCallNode};

#[derive(Debug, Clone)]
pub enum PipelineOperation {
  First { lhs: ExpressionNode, rhs: FunctionCallNode },
  Last { lhs: ExpressionNode, rhs: FunctionCallNode },
}
