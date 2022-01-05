mod typedef;

pub use self::{
  typedef::*,
};

use crate::ast::ExpressionNode;

#[derive(Debug, Clone)]
pub enum Equation {
  TypeDef(TypeDefNode),
  Expression(ExpressionNode),
}

pub type EquationNode = Box<Equation>;
