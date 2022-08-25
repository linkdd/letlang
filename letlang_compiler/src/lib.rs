pub mod prelude;

use self::prelude::*;

use std::{
  path::Path,
  error::Error,
  collections::HashMap,
  cell::RefCell,
};

use letlang_parser::parse_file;
use letlang_ast::{Node, Unit};

pub mod toolchain;
pub mod target;

mod atom_interner;
mod scope;
mod semantic;

mod codegen;

pub use self::{
  toolchain::Toolchain,
  target::Target,
};

pub struct Compiler {
  toolchain: Toolchain,
  target: Target,
}

impl Compiler {
  pub fn new(toolchain: Toolchain, target: Target) -> Self {
    Self {
      toolchain,
      target,
    }
  }

  pub fn compile<P: AsRef<Path>>(
    &mut self,
    inputs: Vec<P>,
    target_dir: P
  ) -> Result<(), Box<dyn Error>> {
    let mut unit_registry: HashMap<String, RefCell<Node<Unit>>> = HashMap::new();
    let mut path_registry: HashMap<String, P> = HashMap::new();

    let mut atom_interner = atom_interner::new();
    let mut scope_arena = scope::ScopeArena::new();

    for input_path in inputs {
      let ast = parse_file(&input_path)?;
      let unit_key = format!("lldep_{}", ast.data.path.join("_"));

      unit_registry.insert(unit_key.clone(), RefCell::new(ast));
      path_registry.insert(unit_key.clone(), input_path);
    }

    let mut model = semantic::Model::new(
      &unit_registry,
      &mut atom_interner,
      &mut scope_arena,
    );
    model.visit().map_err(|(unit_key, err)| {
      let filename = path_registry.get(&unit_key).unwrap();

      CompilationError::new(format!("[{}] {}",
        filename.as_ref().display(),
        err.message,
      ))
    })?;

    let mut generator = codegen::Generator::new(
      &unit_registry,
      &mut atom_interner,
      &scope_arena,
      &self.toolchain,
      &self.target,
    );
    generator.build(target_dir)?;

    Ok(())
  }
}
