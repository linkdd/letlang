use crate::ast::{
  Node,
  types::TypeRef,
  params::CallParam,
};

#[derive(Clone, Debug, PartialEq)]
pub struct EffectDeclStatement {
  pub public: bool,
  pub symbol_name: String,
  pub call_params: Vec<Node<CallParam>>,
  pub return_type: Node<TypeRef>,
}
