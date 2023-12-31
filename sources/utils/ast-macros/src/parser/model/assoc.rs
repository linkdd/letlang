use syn::parse::{Parse, ParseStream, ParseBuffer, Result};
use syn::*;

pub trait AssociatedTypeKeyword {
  fn kw() -> &'static str;
}

pub struct AssociatedType<T: AssociatedTypeKeyword> {
  pub typ: Type,
  _marker: std::marker::PhantomData<T>
}

pub struct OptionalAssociatedType<T: AssociatedTypeKeyword>(
  pub Option<AssociatedType<T>>,
);

impl<T: AssociatedTypeKeyword> AssociatedType<T> {
  fn peek(input: &ParseBuffer) -> bool {
    input.fork().parse::<Self>().is_ok()
  }
}

impl<T: AssociatedTypeKeyword> Parse for AssociatedType<T> {
  fn parse(input: &ParseBuffer) -> Result<Self> {
    let kw = T::kw();

    input.parse::<Token![type]>()?;

    let tok: Ident = input.parse()?;
    if tok != kw {
      return Err(Error::new(tok.span(), format!("expected `{kw}`")));
    }
    input.parse::<Token![=]>()?;
    let typ: Type = input.parse()?;
    input.parse::<Token![;]>()?;

    Ok(Self { typ, _marker: std::marker::PhantomData{} })
  }
}

impl<T: AssociatedTypeKeyword> Parse for OptionalAssociatedType<T> {
  fn parse(input: ParseStream) -> Result<Self> {
    if AssociatedType::<T>::peek(input) {
      let data = input.parse::<AssociatedType<T>>()?;
      Ok(Self(Some(data)))
    }
    else {
      Ok(Self(None))
    }
  }
}

pub struct AssociatedInputData;
pub struct AssociatedInputMeta;
pub struct AssociatedInputExtra;
pub struct AssociatedOutputData;
pub struct AssociatedOutputMeta;
pub struct AssociatedOutput;
pub struct AssociatedError;

impl AssociatedTypeKeyword for AssociatedInputData {
  fn kw() -> &'static str { "InputData" }
}

impl AssociatedTypeKeyword for AssociatedInputMeta {
  fn kw() -> &'static str { "InputMeta" }
}

impl AssociatedTypeKeyword for AssociatedInputExtra {
  fn kw() -> &'static str { "InputExtra" }
}

impl AssociatedTypeKeyword for AssociatedOutputData {
  fn kw() -> &'static str { "OutputData" }
}

impl AssociatedTypeKeyword for AssociatedOutputMeta {
  fn kw() -> &'static str { "OutputMeta" }
}

impl AssociatedTypeKeyword for AssociatedOutput {
  fn kw() -> &'static str { "Output" }
}

impl AssociatedTypeKeyword for AssociatedError {
  fn kw() -> &'static str { "Error" }
}
