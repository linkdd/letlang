use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};

use askama::Template;

#[derive(Template)]
#[template(path = "expression/tailrec_recurse.rs.j2", escape = "none")]
struct TailRecRecurseTemplate {
  args: Vec<String>,
}

impl<'compiler> Generator<'compiler> {
  pub fn gen_tailrec_final(
    &self,
    data: &TailRecFinal,
  ) -> CompilationResult<String> {
    let val_code = self.gen_expression(&data.value)?;
    Ok(format!("Value::TailRecFinal(Box::new({val_code}))"))
  }

  pub fn gen_tailrec_recurse(
    &self,
    location: &LocationInfo,
    data: &TailRecRecurse,
  ) -> CompilationResult<String> {
    let mut args_code = vec![];

    for arg_node in data.args.iter() {
      let arg_code = self.gen_expression(arg_node)?;
      args_code.push(arg_code);
    }

    let context = TailRecRecurseTemplate {
      args: args_code,
    };

    let source_code = context.render().map_err(|e| {
      CompilationError::new_located(
        location,
        format!("Could not generate tailrec recursion source: {}", e),
      )
    })?;

    Ok(source_code)
  }
}
