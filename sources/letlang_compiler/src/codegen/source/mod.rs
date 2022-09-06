pub use super::Generator;
use crate::scope::*;

use std::path::Path;
use std::error::Error;
use std::process::Command;


impl<'compiler> Generator<'compiler> {
  pub fn get_symbol_scope(
    &self,
    scope_name: Option<String>,
    local_scope_id: usize,
  ) -> &Scope {
    match scope_name {
      Some(module_name) => {
        self.get_module_scope(local_scope_id, &module_name)
      },
      None => {
        self.scope_arena.get_scope(local_scope_id)
      },
    }
  }

  pub fn get_module_scope(
    &self,
    local_scope_id: usize,
    module_name: &String,
  ) -> &Scope {
    let scope = self.scope_arena.get_scope(local_scope_id);

    let (_, module_sym) = scope.lookup_symbol(
      self.scope_arena,
      module_name,
    ).unwrap();

    if let SymbolKind::Module { scope_id: module_scope_id } = module_sym {
      let module_scope = self.scope_arena.get_scope(module_scope_id);
      return module_scope;
    }
    else {
      unreachable!("\
        Semantic validation of the AST should have proven \
        that this symbol is a module\
      ");
    }
  }

  pub fn reformat_source<P: AsRef<Path>>(
    &mut self,
    source_path: P,
  ) -> Result<(), Box<dyn Error>> {
    let status = Command::new("rustfmt")
      .arg(format!("--edition={}", self.toolchain.rust_edition))
      .arg(source_path.as_ref().to_path_buf())
      .status()?;

    if !status.success() {
      eprintln!("WARN: rustfmt command exited with non-zero code: {:?}", status.code());
    }

    Ok(())
  }
}

mod exe;
mod module;
