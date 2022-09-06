use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  types::*,
};

use askama::Template;


#[derive(Template)]
#[template(path = "typeref_oneof.rs.j2", escape = "none")]
struct OneOfTypeTemplate {
  typerefs: Vec<String>,
}


impl<'compiler> Generator<'compiler> {
  pub fn gen_typeref_oneof(
    &self,
    location: &LocationInfo,
    data: &OneOfType,
  ) -> CompilationResult<String> {
    let mut typerefs = vec![];

    for typeref_node in data.typerefs.iter() {
      let typeref_code = self.gen_typeref(typeref_node)?;
      typerefs.push(typeref_code);
    }

    let context = OneOfTypeTemplate { typerefs };

    let source_code = context.render().map_err(|e| {
      CompilationError::new_located(
        location,
        format!("Could not generate oneof type source: {}", e),
      )
    })?;

    Ok(source_code)
  }
}
