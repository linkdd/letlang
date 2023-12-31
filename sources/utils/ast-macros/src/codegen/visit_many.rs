use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;

pub fn generate(iterator: Expr, extra: Option<Expr>) -> TokenStream {
  let extra = match extra {
    Some(toks) => quote!{#toks},
    None => quote!{()},
  };

  let tokens = quote!{
    {
      let it = &#iterator;
      let mut outputs = Vec::with_capacity(it.len());

      for child_node in it.iter() {
        let output = self.visit(child_node, #extra)?;
        outputs.push(output);
      }

      outputs
    }
  };

  TokenStream::from(tokens)
}
