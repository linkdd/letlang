pub mod prelude;
mod semantics;
mod phases;
mod codegen;

use self::prelude::*;
use letlang_parser::{
  parse_file,
  ast::{Node, Unit},
};

use std::path::Path;


pub struct Compiler;

impl Compiler {
  pub fn new() -> Self {
    Self {}
  }

  fn error_in_file<P: AsRef<Path>>(
    result: CompilationResult<()>,
    filename: P,
  ) -> CompilationResult<()> {
    result.map_err(|err| {
      CompilationError::new(format!("{}{}",
        filename.as_ref().display(),
        err.message,
      ))
    })
  }

  fn run_phase<P: AsRef<Path>, V: semantics::Visitor>(
    inputs: &Vec<(P, Node<Unit>)>,
    visitor: V,
  ) -> CompilationResult<V> {
    let mut model = semantics::Model::new(visitor);

    for (filename, ast) in inputs.iter() {
      let result = model.visit_unit(ast);
      Self::error_in_file(result, filename)?;
    }

    Ok(model.get_visitor().clone())
  }

  pub fn compile<P: AsRef<Path>>(
    &self,
    inputs: Vec<P>,
    _target_dir: P,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let mut units = vec![];

    for input in inputs {
      let ast = parse_file(&input)?;
      units.push((input, ast));
    }

    let _expr_validator = Self::run_phase(
      &units,
      phases::ExpressionValidatorPhase::new(),
    )?;
    let _atom_interner = Self::run_phase(
      &units,
      phases::AtomInternerPhase::new(),
    )?;

    Ok(())
  }
}
