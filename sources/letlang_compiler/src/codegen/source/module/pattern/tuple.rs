use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};

use askama::Template;


#[derive(Template)]
#[template(path = "pattern/tuple.rs.j2", escape = "none")]
struct TupleTemplate {
  members: Vec<String>,
}

impl<'compiler> Generator<'compiler> {
  pub fn gen_pattern_tuple(
    &self,
    location: &LocationInfo,
    data: &TuplePattern,
  ) -> CompilationResult<String> {
    let mut members = vec![];

    for member_node in data.members.iter() {
      let member_code = self.gen_pattern(member_node)?;
      members.push(member_code);
    }

    let context = TupleTemplate { members };

    let source_code = context.render().map_err(|e| {
      CompilationError::new_located(
        location,
        format!("Could not generate tuple pattern source: {}", e),
      )
    })?;

    Ok(source_code)
  }
}
