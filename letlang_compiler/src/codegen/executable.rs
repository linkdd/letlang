use crate::prelude::*;
use crate::codegen::Context;

use string_interner::Symbol;
use askama::Template;

#[derive(Template)]
#[template(path = "executable.rs.j2")]
struct ExecutableTemplate {
  pub crate_name: String,
  pub builtin_atoms: Vec<BuiltinAtomEntry>,
}

struct BuiltinAtomEntry {
  string: String,
  symbol: usize,
}

pub struct CodeGenerator<'gen, 'ctx> {
  context: &'gen mut Context<'ctx>,
}

impl<'gen, 'ctx> CodeGenerator<'gen, 'ctx> {
  pub fn new(context: &'gen mut Context<'ctx>) -> Self {
    Self { context }
  }

  pub fn generate(&mut self, crate_name: &str) -> CompilationResult<String> {
    let builtin_atoms = vec![
      "@ok",
      "@error",
      "@type_error",
      "@normal"
    ]
      .into_iter()
      .map(|a| BuiltinAtomEntry {
        string: a.to_string(),
        symbol: self.context.atom_interner.get_or_intern(a).to_usize(),
      })
      .collect();

    let context = ExecutableTemplate {
      crate_name: crate_name.to_string(),
      builtin_atoms,
    };

    let source_code = context.render().map_err(|e| {
      CompilationError::new(format!("Could not generate executable source: {}", e))
    })?;

    Ok(source_code)
  }
}
