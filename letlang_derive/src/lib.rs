use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};

struct NodeAttributesParam(syn::Ident);

impl syn::parse::Parse for NodeAttributesParam {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let content;
    syn::parenthesized!(content in input);
    let attrs_type = content.parse()?;
    Ok(Self(attrs_type))
  }
}

#[proc_macro_derive(NodeAttributes, attributes(node_attrs))]
pub fn node_attrs_derive(input: TokenStream) -> TokenStream {
  let DeriveInput { ident, attrs, .. } = syn::parse_macro_input!(input);
  let attribute = attrs
    .iter()
    .filter(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "node_attrs")
    .nth(0);

  let name = ident;

  let gen = match attribute {
    None => {
      quote! {
        use crate::NodeAttributes;

        impl NodeAttributes for #name {
          type Attributes = ();
        }
      }
    },
    Some(parameter_ast) => {
      let NodeAttributesParam(attrs_type) = syn::parse2(parameter_ast.tokens.clone())
        .expect("invalid node_attrs attribute");

      quote! {
        use crate::NodeAttributes;

        impl NodeAttributes for #name {
          type Attributes = #attrs_type;
        }
      }
    }
  };

  gen.into()
}
