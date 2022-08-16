pub mod prelude;
mod semantics;
mod phases;
mod codegen;

use self::prelude::*;
use self::codegen::CodeGenerator;

use letlang_parser::{
  parse_file,
  ast::{Node, Unit},
};

use std::path::Path;


pub struct Compiler {
  runtime_version: String,
  rust_edition: String,
}

impl Compiler {
  pub fn new(runtime_version: String, rust_edition: String) -> Self {
    Self { runtime_version, rust_edition }
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
  ) -> CompilationResult<semantics::Model<V>> {
    let mut model = semantics::Model::new(visitor);

    for (filename, ast) in inputs.iter() {
      let result = model.visit_unit(ast);
      Self::error_in_file(result, filename)?;
    }

    Ok(model)
  }

  pub fn compile<P: AsRef<Path>>(
    &self,
    bin: &str,
    main_module: &str,
    version: &str,
    inputs: Vec<P>,
    target_dir: P,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let mut units = vec![];

    for input in inputs {
      let ast = parse_file(&input)?;
      units.push((input, ast));
    }

    Self::run_phase(
      &units,
      phases::ExpressionValidatorPhase::new(),
    )?;
    let atom_interner_model = Self::run_phase(
      &units,
      phases::AtomInternerPhase::new(),
    )?;

    let mut generator = CodeGenerator::new(
      self.runtime_version.clone(),
      self.rust_edition.clone(),
      target_dir,
    );

    let mut workspace_members = vec!["executable".to_string()];

    for (_, unit) in units.iter() {
      let crate_name = generator.gen_lib_crate(unit, version)?;
      workspace_members.push(format!("modules/{}", crate_name));
    }

    generator.gen_exe_crate(bin, main_module, version)?;
    generator.gen_workspace(workspace_members)?;

    Ok(())
  }
}
