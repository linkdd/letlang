use crate::prelude::*;
use crate::codegen::Context;
use letlang_parser::{ast, ast::Node};

use askama::Template;

#[derive(Template)]
#[template(path = "crate.rs.j2")]
struct CrateTemplate {
  statements: Vec<String>,
}

pub struct CodeGenerator<'gen, 'ctx> {
  context: &'gen Context<'ctx>,
}

impl<'gen, 'ctx> CodeGenerator<'gen, 'ctx> {
  pub fn new(context: &'gen Context<'ctx>) -> Self {
    Self { context }
  }

  pub fn generate(&self, unit: &Node<ast::Unit>) -> CompilationResult<String> {
    let context = CrateTemplate {
      statements: self.gen_unit(unit)?,
    };

    let source_code = context.render().map_err(|e| {
      CompilationError::new(format!("Could not generate executable source: {}", e))
    })?;

    Ok(source_code)
  }
}

mod unit;
mod statement;
