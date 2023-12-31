use ast_core::*;
use ast_macros::*;
use proc_macro2::TokenStream;
use quote::quote;

use llfront::{ast::*, SourceLocation};
use crate::{
  steps::codegen::CodeGenerator,
  steps::scope::{EnvRef, SymbolKind},
  prelude::*,
};


model!{
  impl<'source> Interpreter for CodeGenerator {
    type InputData = Import<(EnvRef, SourceLocation<'source>)>;
    type InputMeta = (EnvRef, SourceLocation<'source>);

    type Output = TokenStream;

    type Error = CompilationError<'source>;

    visit {
      todo!("codegen import");
    }
  }
}
