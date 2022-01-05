use crate::ast::ExpressionNode;

#[derive(Debug, Clone)]
pub enum ArithmeticOperation {
  Add { lhs: ExpressionNode, rhs: ExpressionNode },
  Sub { lhs: ExpressionNode, rhs: ExpressionNode },
  Mul { lhs: ExpressionNode, rhs: ExpressionNode },
  Div { lhs: ExpressionNode, rhs: ExpressionNode },
  Mod { lhs: ExpressionNode, rhs: ExpressionNode },
  Neg { val: ExpressionNode },
}
