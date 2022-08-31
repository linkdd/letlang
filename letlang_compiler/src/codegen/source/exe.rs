use crate::prelude::*;
use super::Generator;

use string_interner::Symbol;
use askama::Template;


#[derive(Template)]
#[template(path = "executable.rs.j2")]
struct ExecutableTemplate {
  pub crate_name: String,
  pub atoms: Vec<AtomTemplate>,
}

struct AtomTemplate {
  repr: String,
  symbol: usize,
}


impl<'compiler> Generator<'compiler> {
  pub fn gen_exe_source(&mut self, crate_name: &str) -> CompilationResult<String> {
    for builtin_atom in vec!["@ok", "@error", "@type_error", "@normal"] {
      self.atom_interner.get_or_intern(builtin_atom);
    }

    let atoms = self.atom_interner.into_iter()
      .map(|(symbol, repr)| AtomTemplate {
        repr: repr.to_string(),
        symbol: symbol.to_usize(),
      })
      .collect();

    let context = ExecutableTemplate {
      crate_name: crate_name.to_string(),
      atoms,
    };

    let source_code = context.render().map_err(|e| {
      CompilationError::new(format!("Could not generate executable source: {}", e))
    })?;

    Ok(source_code)
  }
}
