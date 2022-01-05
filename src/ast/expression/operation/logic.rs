use crate::ast::ExpressionNode;

#[derive(Debug, Clone)]
pub enum LogicOperation {
  And { lhs: ExpressionNode, rhs: ExpressionNode },
  Or { lhs: ExpressionNode, rhs: ExpressionNode },
  Imply { lhs: ExpressionNode, rhs: ExpressionNode },
  Bicond { lhs: ExpressionNode, rhs: ExpressionNode },
  Not { val: ExpressionNode },
}
