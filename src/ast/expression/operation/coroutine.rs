use crate::ast::{ExpressionNode, FunctionCallNode};

#[derive(Debug, Clone)]
pub enum CoroutineOperation {
  Start { func: FunctionCallNode },
  Join { coro: ExpressionNode },
}
