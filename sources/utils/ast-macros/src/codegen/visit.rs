use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;

pub fn generate(child_node: Expr, extra: Option<Expr>) -> TokenStream {
  let extra = match extra {
    Some(toks) => quote!{#toks},
    None => quote!{()},
  };

  let tokens = quote!{
    self.visit(#child_node, #extra)?
  };

  TokenStream::from(tokens)
}
