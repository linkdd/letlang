use proc_macro::TokenStream;
use quote::quote;

use crate::parser::model::ModelInfo;


pub fn generate(model_info: ModelInfo) -> TokenStream {
  let tokens = match model_info {
    ModelInfo::Transformer {
      generics,
      context,
      input_data,
      input_meta,
      input_extra,
      output_data,
      output_meta,
      error,
      body,
    } => {
      let input_extra = match input_extra {
        Some(typ) => quote!{#typ},
        None => quote!{()}
      };

      quote!{
        impl #generics Model<#input_data, #input_meta> for #context {
          type Result = std::result::Result<
            Node<#output_data, #output_meta>,
            #error
          >;
          type Extra = #input_extra;

          fn visit(
            &mut self,
            node: &Node<#input_data, #input_meta>,
            extra: Self::Extra,
          ) -> Self::Result {
            #body
          }
        }
      }
    },
    ModelInfo::Interpreter {
      generics,
      context,
      input_data,
      input_meta,
      input_extra,
      output,
      error,
      body,
    } => {
      let input_extra = match input_extra {
        Some(typ) => quote!{#typ},
        None => quote!{()}
      };

      quote!{
        impl #generics Model<#input_data, #input_meta> for #context {
          type Result = std::result::Result<#output, #error>;
          type Extra = #input_extra;

          fn visit(
            &mut self,
            node: &Node<#input_data, #input_meta>,
            extra: Self::Extra,
          ) -> Self::Result {
            #body
          }
        }
      }
    },
  };

  TokenStream::from(tokens)
}
