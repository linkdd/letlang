use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  statement::*,
};

use askama::Template;


#[derive(Template)]
#[template(path = "statement_function.rs.j2", escape = "none")]
struct FunctionDeclarationTemplate {
  public: bool,
  symbol_name: String,
  type_params: Vec<String>,
  call_params: Vec<CallParamTemplate>,
  call_param_count: usize,
  return_type: String,
  body: Vec<String>,
}

struct CallParamTemplate {
  name: String,
  type_code: String,
}


impl<'compiler> Generator<'compiler> {
  pub fn gen_statement_func(
    &self,
    location: &LocationInfo,
    data: &FuncDeclStatement,
  ) -> CompilationResult<String> {
    let mut type_params = vec![];

    for node in data.type_params.iter() {
      type_params.push(node.data.name.clone());
    }

    let mut call_params = vec![];

    for node in data.call_params.iter() {
      call_params.push(CallParamTemplate {
        name: node.data.param_name.clone(),
        type_code: self.gen_typeref(&node.data.param_type)?,
      });
    }

    let call_param_count = call_params.len();

    let return_type = self.gen_typeref(&data.return_type)?;

    let mut body = vec![];

    for node in data.body.iter() {
      body.push(self.gen_proposition(node)?)
    }

    let context = FunctionDeclarationTemplate {
      public: data.public,
      symbol_name: data.symbol_name.clone(),
      type_params,
      call_params,
      call_param_count,
      return_type,
      body
    };

    let source_code = context.render().map_err(|e| {
      CompilationError::new_located(
        location,
        format!("Could not generate function source: {}", e),
      )
    })?;

    Ok(source_code)
  }
}
