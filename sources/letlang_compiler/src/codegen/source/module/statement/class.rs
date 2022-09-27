use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  statement::*,
};

use askama::Template;


#[derive(Template)]
#[template(path = "statement/class.rs.j2", escape = "none")]
struct ClassDeclarationTemplate {
  public: bool,
  symbol_name: String,
  type_params: Vec<String>,
  cons_param: String,
  constraints: Vec<String>,
}


impl<'compiler> Generator<'compiler> {
  pub fn gen_statement_class(
    &self,
    location: &LocationInfo,
    data: &ClassDeclStatement,
  ) -> CompilationResult<String> {
    let mut type_params = vec![];

    for node in data.type_params.iter() {
      type_params.push(node.data.name.clone());
    }

    let cons_param = self.gen_typeref(&data.cons_param.data.param_type)?;

    let mut constraints = vec![];

    for node in data.constraints.iter() {
      constraints.push(self.gen_proposition(node)?);
    }

    let context = ClassDeclarationTemplate {
      public: data.public,
      symbol_name: data.symbol_name.clone(),
      type_params,
      cons_param,
      constraints,
    };

    let source_code = context.render().map_err(|e| {
      CompilationError::new_located(
        location,
        format!("Could not generate class source: {}", e),
      )
    })?;

    Ok(source_code)
  }
}
