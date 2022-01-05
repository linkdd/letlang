use crate::ast::ExpressionNode;

#[derive(Debug, Clone)]
pub enum ComparisonOperation {
  Less { lhs: ExpressionNode, rhs: ExpressionNode },
  LessEq { lhs: ExpressionNode, rhs: ExpressionNode },
  Eq { lhs: ExpressionNode, rhs: ExpressionNode },
  Ne { lhs: ExpressionNode, rhs: ExpressionNode },
  GreaterEq { lhs: ExpressionNode, rhs: ExpressionNode },
  Greater { lhs: ExpressionNode, rhs: ExpressionNode },
}
