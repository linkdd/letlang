use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};

use askama::Template;


#[derive(Template)]
#[template(path = "expression/func_call.rs.j2", escape = "none")]
struct FunctionCallTemplate {
  func_code: String,
  call_params: Vec<String>,
}


impl<'compiler> Generator<'compiler> {
  pub fn gen_function_call(
    &self,
    location: &LocationInfo,
    data: &FunctionCall,
  ) -> CompilationResult<String> {
    let func_code = self.gen_expression(&data.func)?;
    let mut call_params = vec![];

    for call_param_node in data.params.iter() {
      let call_param_code = self.gen_expression(call_param_node)?;
      call_params.push(call_param_code);
    }

    let context = FunctionCallTemplate {
      func_code,
      call_params,
    };

    let source_code = context.render().map_err(|e| {
      CompilationError::new_located(
        location,
        format!("Could not generate function call source: {}", e),
      )
    })?;

    Ok(source_code)
  }
}
