use ast_core::*;
use ast_macros::*;
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

use llfront::{ast::*, SourceLocation};
use crate::{
  steps::codegen::CodeGenerator,
  steps::scope::{EnvRef, SymbolKind},
  prelude::*,
};


model!{
  impl<'source> Interpreter for CodeGenerator {
    type InputData = NamedDeclaration<(EnvRef, SourceLocation<'source>)>;
    type InputMeta = (EnvRef, SourceLocation<'source>);

    type Output = TokenStream;

    type Error = CompilationError<'source>;

    visit {
      let (env, location) = node.get_meta();
      let data = node.get_data();

      let sym_name = &data.name.get_data().0;
      let sym_kind = {
        env.borrow()
          .get(sym_name.clone())
          .unwrap()
      };

      if data.public {
        self.bmi_syms.push((sym_name.clone(), sym_kind.clone()));
      }

      let visibility_tokens = if data.public {
        quote!{pub}
      }
      else {
        quote!{}
      };

      let mut type_params_tokens = vec![];

      for type_param_node in data.type_params.iter() {
        let name = type_param_node.get_data().0.clone();
        let tokens = quote!{#name : LLClassInstance};
        type_params_tokens.push(tokens);
      }

      let decl_tokens = visit!(&data.declaration, sym_kind.clone());

      let sym_mod_name = format_ident!("symbol_{sym_name}");
      let sym_mod_tokens = quote!{
        #[allow(non_snake_case)]
        #visibility_tokens mod #sym_mod_name {
          extern crate llruntime;
          use llruntime::*;
          use super::*;

          pub struct Object {
            #(#type_params_tokens)*
          }

          #decl_tokens
        }
      };

      let (loc_start, loc_end) = (location.span.start, location.span.end);
      let tokens = quote!{
        sourcemap_begin!(#loc_start, #loc_end);
        #sym_mod_tokens
        sourcemap_end!(#loc_start, #loc_end);
      };

      Ok(tokens)
    }
  }
}

model!{
  impl<'source> Interpreter for CodeGenerator {
    type InputData = Declaration<(EnvRef, SourceLocation<'source>)>;
    type InputMeta = (EnvRef, SourceLocation<'source>);
    type InputExtra = SymbolKind;

    type Output = TokenStream;

    type Error = CompilationError<'source>;

    visit {
      let (env, location) = node.get_meta();
      let data = node.get_data();
      let sym_kind = extra;

      let tokens = match data {
        Declaration::Class(class) => {
          todo!("codegen class");
        },
        Declaration::Effect(effect) => {
          todo!("codegen effect");
        },
        Declaration::Function(func) => {
          let (type_arity, call_arity) = match sym_kind {
            SymbolKind::Function { type_arity, call_arity } => (type_arity, call_arity),
            _ => unreachable!(),
          };

          quote!{
            impl LLFunction for Object {
              fn call(
                &self,
                ctx: LLProcessContext,
                type_params: Vec<LLClassInstance>,
                call_params: Vec<LLValue>,
              ) -> LLContinuation {
                async fn code_block(
                  co: LLCoroutine,
                  ctx: LLProcessContext,
                  type_params: Vec<LLClassInstance>,
                  call_params: Vec<LLValue>,
                ) -> LLValue {
                  ctx.assert_type_arity(&co, #type_arity, type_params.len()).await;
                  ctx.assert_func_arity(&co, #call_arity, call_params.len()).await;

                  todo!()
                }

                LLContinuation::new_boxed(
                  |co| code_block(co, ctx, type_params, call_params)
                )
              }
            }
          }
        },
      };

      let (loc_start, loc_end) = (location.span.start, location.span.end);
      Ok(quote!{
        sourcemap_begin!(#loc_start, #loc_end);
        #tokens
        sourcemap_end!(#loc_start, #loc_end);
      })
    }
  }
}
