use crate::prelude::*;
use crate::semantics::Visitor;
use letlang_parser::ast::expression::*;


#[derive(Clone)]
pub struct ExpressionValidatorPhase;

impl ExpressionValidatorPhase {
  pub fn new() -> Self {
    Self
  }
}

impl Visitor for ExpressionValidatorPhase {
  fn match_node(&self, node: &dyn std::any::Any) -> bool {
    #[allow(unused_parens)]
    (
      node.downcast_ref::<BinaryOperation>().is_some()
      || node.downcast_ref::<GenericResolve>().is_some()
    )
  }

  fn visit_node(&mut self, node: &dyn std::any::Any) -> CompilationResult<()> {
    if let Some(binop) = node.downcast_ref::<BinaryOperation>() {
      match (binop.lhs.data.as_ref(), binop.op, binop.rhs.data.as_ref()) {
        (_, "|>", Expression::FunctionCall(_)) => {
          Ok(())
        },
        (_, "|>", _) => {
          Err(CompilationError::new(
            "pipeline operator's right operand must be a function call".to_string()
          ))
        },
        _ => {
          Ok(())
        }
      }
    }
    else if let Some(genresolve) = node.downcast_ref::<GenericResolve>() {
      match genresolve.symbol.data.as_ref() {
        Expression::Symbol(_) => {
          Ok(())
        },
        _ => {
          Err(CompilationError::new(
            "generic resolve operator expected a symbol".to_string()
          ))
        }
      }
    }
    else {
      Ok(())
    }
  }
}
