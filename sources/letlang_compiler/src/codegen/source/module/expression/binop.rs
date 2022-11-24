use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};

use askama::Template;


#[derive(Template)]
#[template(path = "expression/binop.rs.j2", escape = "none")]
struct BinaryOperationTemplate {
  lhs_code: String,
  rhs_code: String,
  op_func_name: &'static str,
}

impl<'compiler> Generator<'compiler> {
  pub fn gen_binary_op(
    &self,
    location: &LocationInfo,
    data: &BinaryOperation,
  ) -> CompilationResult<String> {
    let lhs_code = self.gen_expression(&data.lhs)?;
    let rhs_code = self.gen_expression(&data.rhs)?;

    let op_func_name = get_op_func_name(data.op);

    let context = BinaryOperationTemplate {
      lhs_code,
      rhs_code,
      op_func_name,
    };

    let source_code = context.render().map_err(|e| {
      CompilationError::new_located(
        location,
        format!("Could not generate binary operation {} source: {}", data.op, e),
      )
    })?;

    Ok(source_code)
  }
}


fn get_op_func_name(op: &str) -> &'static str {
  match op {
    "+"   => "ops::binary::add",
    "-"   => "ops::binary::sub",
    "*"   => "ops::binary::mul",
    "/"   => "ops::binary::div",
    "%"   => "ops::binary::modulo",
    "**"  => "ops::binary::pow",
    "<>"  => "ops::binary::str_concat",
    "="   => "ops::binary::eq",
    "!="  => "ops::binary::ne",
    "<"   => "ops::binary::lt",
    "<="  => "ops::binary::lte",
    ">="  => "ops::binary::gte",
    ">"   => "ops::binary::gt",
    "and" => "ops::binary::and",
    "or"  => "ops::binary::or",
    _ => todo!(),
  }
}
