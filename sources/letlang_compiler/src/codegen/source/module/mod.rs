use crate::prelude::*;
pub use super::Generator;

use letlang_ast::*;

use askama::Template;

#[derive(Template)]
#[template(path = "module.rs.j2", escape = "none")]
struct ModuleTemplate {
  statements: Vec<String>,
}


impl<'compiler> Generator<'compiler> {
  pub fn gen_lib_source(&mut self, unit_node: &Node<Unit>) -> CompilationResult<String> {
    let context = ModuleTemplate {
      statements: self.gen_unit(unit_node)?
    };

    let source_code = context.render().map_err(|e| {
      CompilationError::new(format!("Could not generate module source: {}", e))
    })?;

    Ok(source_code)
  }
}

mod unit;
mod statement;
mod proposition;
mod typeref;
mod expression;
