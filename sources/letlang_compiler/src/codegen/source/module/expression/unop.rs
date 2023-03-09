use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};

use askama::Template;


#[derive(Template)]
#[template(path = "expression/unop.rs.j2", escape = "none")]
struct UnaryOperationTemplate {
  expr_code: String,
  op_func_name: &'static str,
}

impl<'compiler> Generator<'compiler> {
  pub fn gen_unary_op(
    &self,
    location: &LocationInfo,
    data: &UnaryOperation,
  ) -> CompilationResult<String> {
    let expr_code = self.gen_expression(&data.expr)?;

    let op_func_name = get_op_func_name(data.op);

    let context = UnaryOperationTemplate {
      expr_code,
      op_func_name,
    };

    let source_code = context.render().map_err(|e| {
      CompilationError::new_located(
        location,
        format!("Could not generate unary operation {} source: {}", data.op, e),
      )
    })?;

    Ok(source_code)
  }
}


fn get_op_func_name(op: &str) -> &'static str {
  match op {
    "throw" => "ops::unary::throw",
    "-"     => "ops::unary::neg",
    "not"   => "ops::unary::logical_not",
    "~"     => "ops::unary::bitwise_not",
    _ => todo!(),
  }
}
