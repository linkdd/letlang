use crate::ast::{ExpressionNode, FunctionCallNode};

#[derive(Debug, Clone)]
pub enum EffectOperation {
  Perform { effect: FunctionCallNode },
  Throw { reason: ExpressionNode },
}
