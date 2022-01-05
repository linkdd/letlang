use crate::ast::{
  ExpressionNode,
  TypeConstraintNode,
  TypeParamNode,
  StatementNode,
};

#[derive(Debug, Clone)]
pub struct FunctionCall {
  pub func: ExpressionNode,
  pub params: Vec<ExpressionNode>,
}

pub type FunctionCallNode = Box<FunctionCall>;

#[derive(Debug, Clone)]
pub struct LambdaFunction {
  pub return_type: TypeConstraintNode,
  pub params: Vec<TypeParamNode>,
  pub body: Vec<StatementNode>,
}

pub type LambdaFunctionNode = Box<LambdaFunction>;
