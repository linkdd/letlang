use crate::ast::ExpressionNode;

#[derive(Debug, Clone)]
pub enum StreamOperation {
  Inpput { lhs: ExpressionNode, rhs: ExpressionNode },
  Output { lhs: ExpressionNode, rhs: ExpressionNode },
}
