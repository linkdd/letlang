use crate::ast::{EquationNode, ExpressionNode};

#[derive(Debug, Clone)]
pub enum Statement {
  Expression(ExpressionNode),
  Equation(EquationNode),
}

pub type StatementNode = Box<Statement>;
