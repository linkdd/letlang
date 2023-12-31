use ast_core::*;
use ast_macros::*;
use proc_macro2::TokenStream;
use quote::quote;

use llfront::{ast::*, SourceLocation};
use crate::{
  steps::codegen::CodeGenerator,
  steps::scope::EnvRef,
  prelude::*,
};

mod import;
mod decls;

pub use self::{
  import::*,
  decls::*,
};

model!{
  impl<'source> Interpreter for CodeGenerator {
    type InputData = Module<(EnvRef, SourceLocation<'source>)>;
    type InputMeta = (EnvRef, SourceLocation<'source>);

    type Output = (String, TokenStream);

    type Error = CompilationError<'source>;

    visit {
      let (_, location) = node.get_meta();

      let data = node.get_data();

      self.bmi_modpath = data.path.iter()
        .map(|ident_node| ident_node.get_data().0.clone())
        .collect::<Vec<String>>();

      let mod_path = self.bmi_modpath.join("_");
      let crate_name = format!("lldep_{mod_path}");

      let imports_tokens = visit_many!(data.imports);
      let decls_tokens = visit_many!(data.declarations);

      let (loc_start, loc_end) = (location.span.start, location.span.end);
      let tokens = quote!{
        macro_rules! sourcemap_begin {
          ($start:expr, $end:expr) => {};
        }
        macro_rules! sourcemap_end {
          ($start:expr, $end:expr) => {};
        }

        sourcemap_begin!(#loc_start, #loc_end);
        #(#imports_tokens)*
        #(#decls_tokens)*
        sourcemap_end!(#loc_start, #loc_end);
      };

      Ok((crate_name, tokens))
    }
  }
}
