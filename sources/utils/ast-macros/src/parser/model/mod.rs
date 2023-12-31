use syn::parse::{Parse, ParseStream, Result};
use syn::*;

mod assoc;
use self::assoc::*;

pub enum ModelInfo {
  Transformer {
    generics: Generics,
    context: Type,
    input_data: Type,
    input_meta: Type,
    input_extra: Option<Type>,
    output_data: Type,
    output_meta: Type,
    error: Type,
    body: Expr,
  },
  Interpreter {
    generics: Generics,
    context: Type,
    input_data: Type,
    input_meta: Type,
    input_extra: Option<Type>,
    output: Type,
    error: Type,
    body: Expr,
  }
}

impl Parse for ModelInfo {
  fn parse(input: ParseStream) -> Result<Self> {
    input.parse::<Token![impl]>()?;

    let generics: Generics = input.parse()?;

    let model_type: Ident = input.parse()?;
    match model_type {
      t if t == "Transformer" => Self::parse_transformer(input, generics),
      t if t == "Interpreter" => Self::parse_interpreter(input, generics),
      _ => Err(Error::new(model_type.span(), format!("invalid model type: {model_type}"))),
    }
  }
}

impl ModelInfo {
  fn parse_transformer(input: ParseStream, generics: Generics) -> Result<Self> {
    input.parse::<Token![for]>()?;

    let context: Type = input.parse()?;
    let info; braced!(info in input);

    let input_data: AssociatedType<AssociatedInputData> = info.parse()?;
    let input_meta: AssociatedType<AssociatedInputMeta> = info.parse()?;
    let input_extra: OptionalAssociatedType<AssociatedInputExtra> = info.parse()?;
    let output_data: AssociatedType<AssociatedOutputData> = info.parse()?;
    let output_meta: AssociatedType<AssociatedOutputMeta> = info.parse()?;
    let error: AssociatedType<AssociatedError> = info.parse()?;

    let tok: Ident = info.parse()?;
    if tok != "visit" {
      return Err(Error::new(tok.span(), "expected `visit`"));
    }

    let body: Expr = info.parse()?;

    Ok(Self::Transformer {
      generics,
      context,
      input_data: input_data.typ,
      input_meta: input_meta.typ,
      input_extra: input_extra.0.map(|t| t.typ),
      output_data: output_data.typ,
      output_meta: output_meta.typ,
      error: error.typ,
      body,
    })
  }

  fn parse_interpreter(input: ParseStream, generics: Generics) -> Result<Self> {
    input.parse::<Token![for]>()?;

    let context: Type = input.parse()?;
    let info; braced!(info in input);

    let input_data: AssociatedType<AssociatedInputData> = info.parse()?;
    let input_meta: AssociatedType<AssociatedInputMeta> = info.parse()?;
    let input_extra: OptionalAssociatedType<AssociatedInputExtra> = info.parse()?;
    let output: AssociatedType<AssociatedOutput> = info.parse()?;
    let error: AssociatedType<AssociatedError> = info.parse()?;

    let tok: Ident = info.parse()?;
    if tok != "visit" {
      return Err(Error::new(tok.span(), "expected `visit`"));
    }

    let body: Expr = info.parse()?;

    Ok(Self::Interpreter {
      generics,
      context,
      input_data: input_data.typ,
      input_meta: input_meta.typ,
      input_extra: input_extra.0.map(|t| t.typ),
      output: output.typ,
      error: error.typ,
      body,
    })
  }
}
