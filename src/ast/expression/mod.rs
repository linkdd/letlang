mod literal;
mod variable;
mod function;
mod operation;

pub use self::{
  literal::*,
  variable::*,
  function::*,
  operation::*,
};

#[derive(Debug, Clone)]
pub enum Expression {
  Literal(LiteralNode),
  VariableName(VariableNameNode),
  FunctionCall(FunctionCallNode),
  LambdaFunction(LambdaFunctionNode),
  Operation(OperationNode),
}

pub type ExpressionNode = Box<Expression>;
