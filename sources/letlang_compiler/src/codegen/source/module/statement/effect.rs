use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  statement::*,
};

use askama::Template;


#[derive(Template)]
#[template(path = "statement_effect.rs.j2", escape = "none")]
struct EffectDeclarationTemplate {
  public: bool,
  symbol_name: String,
  call_params: Vec<CallParamTemplate>,
  call_param_count: usize,
  return_type: String,
}

struct CallParamTemplate {
  type_code: String,
}


impl<'compiler> Generator<'compiler> {
  pub fn gen_statement_effect(
    &self,
    location: &LocationInfo,
    data: &EffectDeclStatement,
  ) -> CompilationResult<String> {
    let mut call_params = vec![];

    for node in data.call_params.iter() {
      call_params.push(CallParamTemplate {
        type_code: self.gen_typeref(&node.data.param_type)?,
      });
    }

    let call_param_count = call_params.len();

    let return_type = self.gen_typeref(&data.return_type)?;

    let context = EffectDeclarationTemplate {
      public: data.public,
      symbol_name: data.symbol_name.clone(),
      call_params,
      call_param_count,
      return_type,
    };

    let source_code = context.render().map_err(|e| {
      CompilationError::new_located(
        location,
        format!("Could not generate effect source: {}", e),
      )
    })?;

    Ok(source_code)
  }
}
