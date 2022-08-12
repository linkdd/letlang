pub mod prelude;
mod semantics;
mod codegen;

use self::prelude::*;

pub struct Compiler;

impl Compiler {
  pub fn new() -> Self {
    Self {}
  }

  pub fn compile(&self, inputs: Vec<String>, target_dir: String) -> CompilationResult<()> {
    todo!();
  }
}
