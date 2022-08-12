use crate::prelude::*;
pub use super::Model;

use letlang_parser::ast::{
  Node,
  expression::Expression,
  types::TypeRef,
};


impl Model {
  pub fn visit_expression(&mut self, node: &Node<Expression>) -> CompilationResult<()> {
    let result = match node.data.as_ref() {
      Expression::Literal(val) => {
        self.visit_literal(val)
      },
      Expression::Symbol(path) => todo!(),
      Expression::Structure { members } => todo!(),
      Expression::Tuple { members } => todo!(),
      Expression::List { items } => todo!(),
      Expression::PatternMatch { lhs, rhs } => todo!(),
      Expression::MemberAccess { lhs, rhs } => {
        self.visit_member_access(lhs, rhs)
      },
      Expression::GenericResolve { symbol, type_params } => {
        self.visit_generic_resolve(symbol, type_params)
      },
      Expression::EffectCall { effect_name, params } => todo!(),
      Expression::FunctionCall { func, params } => todo!(),
      Expression::SpawnCall { func, params } => todo!(),
      Expression::UnaryOperation { op, expr } => {
        self.visit_unary_operation(op, expr)
      },
      Expression::BinaryOperation { lhs, op, rhs } => {
        self.visit_binary_operation(lhs, op, rhs)
      },
      Expression::TypeCheck { lhs, rhs } => todo!(),
      Expression::Loop { label, body } => todo!(),
      Expression::Break { label, value } => todo!(),
      Expression::FlowMatch { expr, cases } => todo!(),
      Expression::FlowConditional { cases, else_case } => todo!(),
      Expression::Receive { cases, after } => todo!(),
    };

    result.map_err(|err| {
      if !err.is_located() {
        CompilationError::new_located(&node.location, err.message)
      }
      else {
        err
      }
    })
  }

  fn visit_member_access(&mut self, lhs: &Node<Expression>, rhs: &str) -> CompilationResult<()> {
    self.visit_expression(lhs)?;
    Ok(())
  }

  fn visit_generic_resolve(&mut self, symbol: &Node<Expression>, type_params: &Vec<Node<TypeRef>>) -> CompilationResult<()> {
    match symbol.data.as_ref() {
      Expression::Symbol(..) => {
        Ok(())
      },
      _ => {
        Err(CompilationError::new("generic resolve operator expected a symbol".to_string()))
      },
    }?;

    self.visit_expression(symbol)?;

    for type_param in type_params.iter() {
      self.visit_typeref(type_param)?;
    }

    Ok(())
  }

  fn visit_unary_operation(&mut self, op: &str, expr: &Node<Expression>) -> CompilationResult<()> {
    self.visit_expression(expr)
  }

  fn visit_binary_operation(&mut self, lhs: &Node<Expression>, op: &str, rhs: &Node<Expression>) -> CompilationResult<()> {
    match (lhs.data.as_ref(), op, rhs.data.as_ref()) {
      (_, "|>", Expression::FunctionCall { .. }) => {
        Ok(())
      },
      (_, "|>", _) => {
        Err(CompilationError::new("pipeline operator's righthand side operand must be a function call".to_string()))
      },
      _ => {
        Ok(())
      }
    }?;

    self.visit_expression(lhs)?;
    self.visit_expression(rhs)?;

    Ok(())
  }
}

mod literal;
