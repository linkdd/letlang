use nonempty::NonEmpty;
use proc_macro2::TokenStream;

use ast_core::*;
use llfront::{ast::*, SourceLocation};

use crate::{
  prelude::*,
  steps::scope::{EnvRef, SymbolKind},
};

mod context;
mod model;

use self::{
  context::CodeGenerator,
  model::*,
};

pub fn eval<'source>(
  /* bmi */
  ast: &AST<(EnvRef, SourceLocation<'source>)>
) -> Result<'source, (String, llbmi::BinaryModuleInterface, TokenStream)> {
  let mut context = CodeGenerator::new(/* bmi */);
  let (crate_name, code) = context.visit(&ast.0, ())?;

  let bmi = llbmi::BinaryModuleInterface {
    path: NonEmpty::from_vec(context.bmi_modpath).unwrap(),
    symbols: context.bmi_syms.into_iter().map(|(name, sym)| {
      match sym {
        SymbolKind::Class { type_arity, .. } => {
          llbmi::Symbol::Class { name, type_arity }
        },
        SymbolKind::Function { type_arity, call_arity } => {
          llbmi::Symbol::Function { name, type_arity, call_arity }
        },
        SymbolKind::Effect { type_arity, call_arity } => {
          llbmi::Symbol::Effect { name, type_arity, call_arity }
        },
        _ => {
          unreachable!()
        },
      }
    }).collect(),
  };

  Ok((crate_name, bmi, code))
}
