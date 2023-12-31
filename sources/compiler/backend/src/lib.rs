pub mod prelude;
pub mod steps;

use proc_macro2::TokenStream;
use quote::quote;

use self::prelude::*;
use llfront::{AST, SourceLocation};
use llbmi::BinaryModuleInterface;


pub fn compile_lib<'source>(
  ast: AST<SourceLocation<'source>>,
) -> Result<(String, BinaryModuleInterface, String)> {
  let (root_env, ast) = steps::scope::transform(&ast)?;
  let (crate_name, bmi, code) = steps::codegen::eval(&ast)?;
  Ok((crate_name, bmi, format!("{code}")))
}

pub fn compile_exe<'source>(
  ast: AST<SourceLocation<'source>>,
) -> Result<(String, BinaryModuleInterface, String)> {
  let (root_env, ast) = steps::scope::transform(&ast)?;
  let (crate_name, bmi, code) = steps::codegen::eval(&ast)?;

  let code = quote!{
    #code

    pub fn main() {
      todo!("main")
    }
  };

  Ok((crate_name, bmi, format!("{code}")))
}
