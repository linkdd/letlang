use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  types::*,
};

use askama::Template;


#[derive(Template)]
#[template(path = "typeref_tuple.rs.j2", escape = "none")]
struct TupleTypeTemplate {
  members: Vec<String>,
}


impl<'compiler> Generator<'compiler> {
  pub fn gen_typeref_tuple(
    &self,
    location: &LocationInfo,
    data: &TupleDefinition,
  ) -> CompilationResult<String> {
    let mut members = vec![];

    for member_node in data.members.iter() {
      let member_code = self.gen_typeref(member_node)?;
      members.push(member_code);
    }

    let context = TupleTypeTemplate { members };

    let source_code = context.render().map_err(|e| {
      CompilationError::new_located(
        location,
        format!("Could not generate tuple type source: {}", e),
      )
    })?;

    Ok(source_code)
  }
}
