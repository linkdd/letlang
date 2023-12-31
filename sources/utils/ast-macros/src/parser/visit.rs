use syn::parse::{Parse, ParseStream, Result};
use syn::*;

pub struct VisitArgs {
  pub first: Expr,
  pub extra: Option<Expr>,
}

impl Parse for VisitArgs {
  fn parse(input: ParseStream) -> Result<Self> {
    let first: Expr = input.parse()?;

    let extra = if !input.is_empty() {
      input.parse::<Token![,]>()?;
      Some(input.parse::<Expr>()?)
    }
    else {
      None
    };

    Ok(Self { first, extra })
  }
}
