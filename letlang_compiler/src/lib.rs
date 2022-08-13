pub mod prelude;
mod semantics;
mod phases;
mod codegen;

use self::prelude::*;
use letlang_parser::{
  parse_file,
  ast::{Node, Unit},
};


pub struct Compiler;

impl Compiler {
  pub fn new() -> Self {
    Self {}
  }

  fn error_in_file(result: CompilationResult<()>, filename: &str) -> CompilationResult<()> {
    result.map_err(|err| {
      CompilationError::new(format!("{}{}", filename, err.message))
    })
  }

  pub fn compile(
    &self,
    inputs: Vec<String>,
    target_dir: String,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let mut units = vec![];

    for input in inputs {
      let ast = parse_file(&input)?;
      units.push((input, ast));
    }

    self.intern_atoms(&units)?;

    Ok(())
  }

  fn intern_atoms(&self, inputs: &Vec<(String, Node<Unit>)>) -> CompilationResult<()> {
    let mut model = semantics::Model::new(
      phases::AtomInternerPhase::new()
    );

    for (filename, ast) in inputs.iter() {
      let result = model.visit_unit(ast);
      Self::error_in_file(result, filename)?;
    }

    Ok(())
  }
}
