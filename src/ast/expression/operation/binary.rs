use crate::ast::ExpressionNode;

#[derive(Debug, Clone)]
pub enum BinaryOperation {
  And { lhs: ExpressionNode, rhs: ExpressionNode },
  Or { lhs: ExpressionNode, rhs: ExpressionNode },
  Xor { lhs: ExpressionNode, rhs: ExpressionNode },
  LShift { lhs: ExpressionNode, rhs: ExpressionNode },
  RShift { lhs: ExpressionNode, rhs: ExpressionNode },
  Not { val: ExpressionNode },
}
